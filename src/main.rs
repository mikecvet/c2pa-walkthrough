use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use c2pa::{create_signer, Ingredient, Manifest, ManifestStore, SigningAlg};
use c2pa::assertions::{c2pa_action, Action, Actions, CreativeWork, Exif, SchemaDotOrgPerson};
use chrono::prelude::{DateTime, Utc};
use clap::{arg, Command};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
/* Example struct, used as labeled assertion data */
struct MediaData {
    n: usize,
    m: usize,
    desc: String,
    ts: u64
}

impl MediaData {
    fn new(n: usize, m: usize, desc: String) -> MediaData {
        MediaData {
            n: n,
            m: m,
            desc: desc,
            ts: SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs()
        }
    }
}

/**
 * Creates a new `Manifest` for an image file. Represents a set of
 * actions performed when creating a new media file, for example, after
 * a digital image is taken.
 */
fn 
create_new_manifest (src_path: &String, dest_path: &String) -> Result<(), c2pa::Error> {
    let now: DateTime<Utc> = SystemTime::now().into();

    // ISO 8601 date and time format
    let now_string = now.to_rfc3339();

    // Initialized new Manifest with claim generator user agent string
    let mut manifest = Manifest::new("mikes-c2pa-test-code/0.1".to_owned());

    // A new `CreativeWork`, defined in schema.org https://schema.org/CreativeWork
    // This represents the media created by the user, whose details are added to the 
    // `CreativeWork` as the author.
    let creative_work = CreativeWork::new()
        .add_author(
            SchemaDotOrgPerson::new()
                .set_name("Mike Cvet")
                  .expect("set name")
                .set_identifier("mikecvet")
                  .expect("set identifier")
        )?;

    // A new `Action` reflecting the creation of the `CreativeWork`.    
    let created = Actions::new()
        .add_action(
            Action::new(c2pa_action::CREATED)
                .set_source_type("https://cv.iptc.org/newscodes/digitalsourcetype/digitalCapture".to_owned())
                .set_software_agent("mikes-c2pa-test-code/0.1")
                .set_when(now_string.clone())
        );

    // Example Exif data to be embedded into the `Manifest`    
    let exif = Exif::from_json_str(
        r#"{
        "@context" : {
          "exif": "http://ns.adobe.com/exif/1.0/"
        },
        "exif:GPSLatitude": "48,15.7068N",
        "exif:GPSLongitude": "16,15.9996W",
        "exif:GPSTimeStamp": "2023-08-23T19:12:45Z"
        }"#,
    ).expect("exif");

    // This is a verified credential string; see https://www.w3.org/TR/vc-data-model
    let vc = r#"{
        "@context": [
            "https://www.w3.org/2018/credentials/v1",
            "http://schema.org"
        ],
        "type": [
            "VerifiableCredential",
            "NPPACredential"
        ],
        "issuer": "https://nppa.org/",
        "credentialSubject": {
            "id": "did:nppa:eb1bb9934d9896a374c384521410c7f14",
            "name": "Bob Ross",
            "memberOf": "https://nppa.org/"
        },
        "proof": {
            "type": "RsaSignature2018",
            "created": "2021-06-18T21:19:10Z",
            "proofPurpose": "assertionMethod",
            "verificationMethod":
            "did:nppa:eb1bb9934d9896a374c384521410c7f14#_Qq0UL2Fq651Q0Fjd6TvnYE-faHiOpRlPVQcY_-tA4A",
            "jws": "eyJhbGciOiJQUzI1NiIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19DJBMvvFAIC00nSGB6Tn0XKbbF9XrsaJZREWvR2aONYTQQxnyXirtXnlewJMBBn2h9hfcGZrvnC1b6PgWmukzFJ1IiH1dWgnDIS81BH-IxXnPkbuYDeySorc4QU9MJxdVkY5EL4HYbcIfwKj6X4LBQ2_ZHZIu1jdqLcRZqHcsDF5KKylKc1THn5VRWy5WhYg_gBnyWny8E6Qkrze53MR7OuAmmNJ1m1nN8SxDrG6a08L78J0-Fbas5OjAQz3c17GY8mVuDPOBIOVjMEghBlgl3nOi1ysxbRGhHLEK4s0KKbeRogZdgt1DkQxDFxxn41QWDw_mmMCjs9qxg0zcZzqEJw"
        }
    }"#;

    // Sets some basics of the manifest
    manifest.set_title("title");
    manifest.set_format("image/jpeg");

    // Adds assertions about the content to the manifest
    manifest.add_assertion(&creative_work)?;
    manifest.add_assertion(&created)?;
    manifest.add_assertion(&exif)?;

    // Add custom data until this label
    manifest.add_labeled_assertion("org.contentauth.test", &MediaData::new(128, 256, "descriptive string".to_string()))?;

    // For some reason, this causes manifest embedding to fail. AFAICT this is a valid formatting for verified credentials, pulled
    // from SDK test code. 
    // manifest.add_verifiable_credential(&vc.to_string())?;

    let source = PathBuf::from(src_path);
    let dest = PathBuf::from(dest_path);

    // Create a ps256 signer using certs and key files
    let signcert_path = "../c2pa-rs/sdk/tests/fixtures/certs/ps256.pub";
    let pkey_path = "../c2pa-rs/sdk/tests/fixtures/certs/ps256.pem";
    let signer = create_signer::from_files(signcert_path, pkey_path, SigningAlg::Ps256, None);

    // Signs and embeds the manifest into the destination file
    manifest.embed(&source, &dest, &*signer.unwrap())?;

    Ok(())
}

fn 
edit_media_with_action (src_path: &String, dest_path: &String, action: &str) -> Result<(), c2pa::Error> {
    // Manifests cannot be edited. To modify the contents of the manifest store, pull in earlier versions of the content
    // and its manifest as an ingredient.
    let parent = Ingredient::from_file(src_path)?;

    let mut manifest = Manifest::new("mikes-c2pa-test-code/0.1".to_owned());

    let now: DateTime<Utc> = SystemTime::now().into();
    let now_string = now.to_rfc3339();

    // also add an action that we opened the file
    let actions = Actions::new()
        .add_action(
            Action::new(c2pa_action::OPENED)
                .set_parameter("identifier", parent.instance_id().to_owned())
                .expect("set identifier")
                .set_reason("editing")
                .set_software_agent("mikes-c2pa-test-code/0.1")
                .set_when(now_string.clone())
        )
        .add_action(
            Action::new(action)
                .set_parameter("identifier", parent.instance_id().to_owned())
                .expect("set identifier")
                .set_reason("editing")
                .set_source_type("https://cv.iptc.org/newscodes/digitalsourcetype/minorHumanEdits".to_owned())
                .set_software_agent("mikes-c2pa-test-code/0.1")
                .set_when(now_string.clone())
        );

    manifest.set_parent(parent)?;
    manifest.add_assertion(&actions)?;

    // Create a ps256 signer using certs and key files
    let signcert_path = "../c2pa-rs/sdk/tests/fixtures/certs/ps256.pub";
    let pkey_path = "../c2pa-rs/sdk/tests/fixtures/certs/ps256.pem";
    let signer = create_signer::from_files(signcert_path, pkey_path, SigningAlg::Ps256, None);

    manifest.embed(&src_path, &dest_path, &*signer.unwrap())?;

    Ok(())
}

fn 
read_manifest (path: &String) -> Result<(), c2pa::Error> {

    let manifest_store = ManifestStore::from_file(path)?;

    match manifest_store.validation_status() {
        Some(statuses) if !statuses.is_empty() => {
            println!("Loading manifest resulted in validation errors:");
            for status in statuses {
                println!("Validation status code: {}", status.code());
            }

            panic!("data validation errors");
        },
        _ => ()
    }

    println!("manifest store: {}", manifest_store);

    // active manifest is the most recently added manifest in the store.
    let manifest = manifest_store.get_active().unwrap();
    println!("active manifest: {}", manifest);

    println!("all manifests:\n----------------------");
    for iter in manifest_store.manifests().iter() {
        println!("manifest {},{}", iter.0, iter.1);
    }

    Ok(())
}

fn 
main() {

    // By default, just run with --path test_file.jpg

    let matches = Command::new("c2pa-walkthrough")
    .version("0.1")
    .about("learning the c2pa-rs SDK")
    .arg(arg!(--path <VALUE>).required(false))
    .get_matches();

    let path = matches.get_one::<String>("path");

    match path {
        Some(file_path) => {
            let file_path_regex = Regex::new(r"(.+)\.([a-zA-Z]+)").unwrap();
            let captures = file_path_regex.captures(&file_path).unwrap();

            // filename prefix; output media files (with added manifests) to to a new file with a suffix added.
            // For exmaple, destination file would be "test_file_c2pa.jpg" given an input of "test_file.jpg"
            let mut file_with_manifest = captures.get(1).unwrap().as_str().to_owned();

            // suffix for output file
            file_with_manifest.push_str("_c2pa");

            // filename extension
            file_with_manifest.push_str(".");
            file_with_manifest.push_str(captures.get(2).unwrap().as_str());

            match create_new_manifest(file_path, &file_with_manifest) {
                Ok(_) => (),
                Err(e) => panic!("error creating manifest: {}", e)
            }

            match (
                edit_media_with_action(&file_with_manifest, &file_with_manifest, c2pa_action::CROPPED), 
                edit_media_with_action(&file_with_manifest, &file_with_manifest, c2pa_action::FILTERED), 
                edit_media_with_action(&file_with_manifest, &file_with_manifest, c2pa_action::COLOR_ADJUSTMENTS)
            ) {
                (Ok(()), Ok(()), Ok(())) => {
                    read_manifest(&file_with_manifest).expect("manifest should be printed to stdout");
                },
                (Err(e), _, _) => panic!("cropping edit failed with {}", e),
                (_, Err(e), _) => panic!("filtering edit failed with {}", e),
                (_, _, Err(e)) => panic!("color adjustment edit failed with {}", e),
            };
        }
        _ => {
            println!("provide a path to a media file via --path <arg>");
        }
    }
}

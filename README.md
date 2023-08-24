# C2PA Simple Walkthrough

This is a simple walkthrough of the C2PA Rust SDK from [contentauth/c2pa-rs](https://github.com/contentauth/c2pa-rs). 

    ~>> ./target/release/c2pa-walkthrough --path ./test_file.jpg 

    manifest store: {
        "active_manifest": "urn:uuid:aabda386-2835-455e-9773-a750ff8fc7a4",
        "manifests": {
            "urn:uuid:aabda386-2835-455e-9773-a750ff8fc7a4": {
                "claim_generator": "mikes-c2pa-test-code/0.1 c2pa-rs/0.25.2",
                "title": "test_file_c2pa.jpg",
                "format": "image/jpeg",
                "instance_id": "xmp:iid:2341e08c-4482-42a3-9eea-558696ba94e2",

    [ ... ]

    "assertions": [
    {
        "label": "c2pa.actions",
        "data": {
            "actions": [
              {
                "action": "c2pa.opened",
                "parameters": {
                    "identifier": "xmp:iid:4af197dd-7b85-4cbe-ab93-c2d124a90b4c"
                },
                "reason": "editing",
                "softwareAgent": "mikes-c2pa-test-code/0.1",
                "when": "2023-08-24T03:20:16.857741+00:00"
              },
          {
            "action": "c2pa.cropped",
            "digitalSourceType": "https://cv.iptc.org/newscodes/digitalsourcetype/minorHumanEdits",
            "parameters": {
              "identifier": "xmp:iid:4af197dd-7b85-4cbe-ab93-c2d124a90b4c"
            },
            "softwareAgent": "mikes-c2pa-test-code/0.1",
            "when": "2023-08-24T03:20:16.857741+00:00"
          }
        ]
      }
    }
  ],
  "signature_info": {
    "issuer": "C2PA Test Signing Cert",
    "cert_serial_number": "720724073027128164015125666832722375746636448153"
  },
  "label": "urn:uuid:af72af31-6201-44c2-b506-8875ed22c788"
}
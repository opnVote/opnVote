This module contains general tooling used for opnVote development. 

Rust tools are placed in `src/bin` (one file per tool) and can then be run with `cargo run --bin "NAME"`. For other tools refer to their description at the top of their file or in this README.

# `generate_keypair.rs`
How to run: `cargo run --bin generate_keypair`

Generates a keypair in ed25519 format and places them as `ed25519.pk8`/`ed25519-pub.raw`. Why not use OpenSSL? Because I wasn't able to figure out how to build PKCS#8 v2 with OpenSSL so that `ring` liked it.  

# `generate_mtls_chain.sh`
How to run: `bash generate_mtls_chain.sh`

Generates a full certificate chain including all authorities and bundles for the mTLS authentication used by opnVote. Do NOT use this in an actual production scenario !!!

Will save all files into `resources/certs`

# `mtls_server.sh`
How to run: `bash mtls_server.sh`

Spins up a simple OpenSSL server with the certs generated from `generate_mtls_chain.sh` to test the chain. Use in conjunction with `mtls_client.sh`.

# `mtls_client.sh`
How to run: `bash mtls_client.sh`

Sends a request with the certs from `generate_mtls_chain.sh` to the OpenSSL server spun up with `mtls_server.sh` to test the cert chain.
# Setting up a development instance

Setting up `opnVote` for development in-project is pretty easy. This guide will walk you through the simplest configuration for opnVote: Backbone, one controller and a single vote machine.

## 1. Prerequisites
- A linux installation 
- A valid rust toolchain with `cargo` and `rustc`
- `openssl`

## 1. Generating a TLS Chain

This can be done in one step using `bash tooling/src/generate_mtls_chain.sh`. They will be saved in `resources/certs`

## 2. Generating a keypair

This too can be done with a script: `cargo run --bin generate_keypair`, which will save them in project root.
Move these into `resources/keys/ed25519{.pk8, -public.raw}`

## 3. Starting everything

Just start the separate crates in the following order:
1. `opnvote-backbone`
2. `opnvote-vote-controller`
3. `opnvote-vote-machine`
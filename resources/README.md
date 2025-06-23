These are resources used for test environments of opnVote.

> [!CAUTION]  
> It is imperative that you do not use any of this in a live environment!!!  
> In doing this you are severely at risk of exposing yourself to all sorts of attack vectors including compromising your  

# `certs/`
This is a full cert stack with CA/Keys used for testing the m2m-communication which is done over mTLS including full-chain-bundles for the rust applications and pkcs12-bundles for browsers. Generated with `tooling/src/generate_mtls_chain.sh`

# `keys/`
Keys for testing purposes used for vote signing etc.

# `electoral_list.json`
Sample list of candidates and parties

# `revoked_tokens.txt`
Current way to keep track of revoked tokens, will be replaced in the very near future.
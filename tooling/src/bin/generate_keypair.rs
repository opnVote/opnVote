/*
opnVote - Reliable, verifiable and secure electronic voting systems
Copyright (C)  2025  Max Bossing <max@bossi.ng>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::fs::File;
use std::io::Write;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};

// Generate an ed25519 keypair that ring can use
// Because ring is very picky about how the keypair is serialized, and for the love of god I wasnt able to generate DER version 01 keys with openssl
// IDK if we want to keep this format anyway, so this is fine for now

// ONLY TO BE USED FOR TESTING; ACTUAL KEYS SHOULD BE GENERATED IN A KEY CEREMONY

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rng = SystemRandom::new();

    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    
    let mut priv_file = File::create("ed25519.pk8")?;
    priv_file.write_all(pkcs8.as_ref())?;

    let pub_bytes = key_pair.public_key().as_ref(); // &[u8; 32]
    let mut pub_file = File::create("ed25519-public.raw")?;
    pub_file.write_all(pub_bytes)?;
    
    println!("Wrote keypair to ed25519.pk8 and ed25519-public.raw!");
    Ok(())
}
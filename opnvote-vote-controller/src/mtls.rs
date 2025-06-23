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
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{RootCertStore, ServerConfig};
use rustls::server::WebPkiClientVerifier;

fn load_certs(path: &str) -> Vec<CertificateDer<'static>> {
    let certfile = std::fs::read(path).expect("cannot read cert file");
    rustls_pemfile::certs(&mut &*certfile)
        .map(|e| e.unwrap())
        .into_iter()
        .map(CertificateDer::from)
        .collect()
}

fn load_single_key(path: &str) -> PrivateKeyDer<'static> {
    let keyfile = std::fs::read(path).expect("cannot read key file");
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut &*keyfile)
        .into_iter()
        .map(|e| e.unwrap())
        .map(PrivateKeyDer::from)
        .collect::<Vec<_>>();

    keys.remove(0)  // panic if none; warn if multiple
}

pub fn tls_config() -> ServerConfig {
    let server_certs = load_certs("resources/certs/server-full-chain.crt");
    let server_key   = load_single_key("resources/certs/server.key");

    let mut root_store = RootCertStore::empty();
    for ca in &server_certs {
        root_store.add(ca.clone()).expect("failed to add CA");
    }

    let client_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .expect("failed to build client verifier");

    let mut config = ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(server_certs, server_key)
        .expect("invalid server cert/key");

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    config
}
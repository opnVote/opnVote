#!/usr/bin/env bash

#opnVote - Reliable, verifiable and secure electronic voting systems
#Copyright (C)  2025  Max Bossing <max@bossi.ng>
#
#This program is free software: you can redistribute it and/or modify
#it under the terms of the GNU Affero General Public License as
#published by the Free Software Foundation, either version 3 of the
#License, or (at your option) any later version.
#
#This program is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU Affero General Public License for more details.
#
#You should have received a copy of the GNU Affero General Public License
#along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Generates a mtls chain including all certificate authorities
# Per default the server and client both run on localhost
# no args, will create the chain in resources/certs
# A great resource for this is https://gist.github.com/Soarez/9688998
# ONLY TO BE USED FOR TESTING; ACTUAL CERTIFICATES SHOULD BE GENERATED IN A KEY CEREMONY

CERTS_PATH="../../resources/certs"

# generate root ca
openssl genrsa -out "$CERTS_PATH/ca-root.key" 4096
openssl req -x509 -new -nodes -key "$CERTS_PATH/ca-root.key" -sha256 -days 3650 -out "$CERTS_PATH/ca-root.crt" -subj "/C=DE/ST=Berlin/L=Berlin/O=opnVote/OU=testing/CN=ca-intermediate-server"

# generate intermediate ca for server
openssl genrsa -out "$CERTS_PATH/ca-intermediate-server.key" 4096
openssl req -new -key "$CERTS_PATH/ca-intermediate-server.key" -out "$CERTS_PATH/ca-intermediate-server.csr" -subj "/C=DE/ST=Berlin/L=Berlin/O=opnVote/OU=testing/CN=ca-intermediate-server"
openssl x509 -req -in "$CERTS_PATH/ca-intermediate-server.csr" -CA "$CERTS_PATH/ca-root.crt" -CAkey "$CERTS_PATH/ca-root.key" -CAcreateserial -out "$CERTS_PATH/ca-intermediate-server.crt" -days 3650 -sha256 -extfile <(echo "basicConstraints=critical,CA:TRUE")

# generate intermediate ca for client
openssl genrsa -out "$CERTS_PATH/ca-intermediate-client.key" 4096
openssl req -new -key "$CERTS_PATH/ca-intermediate-client.key" -out "$CERTS_PATH/ca-intermediate-client.csr" -subj "/C=DE/ST=Berlin/L=Berlin/O=opnVote/OU=testing/CN=ca-intermediate-client"
openssl x509 -req -in "$CERTS_PATH/ca-intermediate-client.csr" -CA "$CERTS_PATH/ca-root.crt" -CAkey "$CERTS_PATH/ca-root.key" -CAcreateserial -out "$CERTS_PATH/ca-intermediate-client.crt" -days 3650 -sha256 -extfile <(echo "basicConstraints=critical,CA:TRUE")

# generate server certificates
openssl genrsa -out "$CERTS_PATH/server.key" 4096
openssl req -new -key "$CERTS_PATH/server.key" -out "$CERTS_PATH/server.csr" -subj "/C=DE/ST=Berlin/L=Berlin/O=opnVote/OU=testing/CN=localhost"
openssl x509 -req -in "$CERTS_PATH/server.csr" -CA "$CERTS_PATH/ca-intermediate-server.crt" -CAkey "$CERTS_PATH/ca-intermediate-server.key" -CAcreateserial -out "$CERTS_PATH/server.crt" -days 3650 -sha256

# generate client certificates
openssl genrsa -out "$CERTS_PATH/client.key" 4096
openssl req -new -key "$CERTS_PATH/client.key" -out "$CERTS_PATH/client.csr" -subj "/C=DE/ST=Berlin/L=Berlin/O=opnVote/OU=testing/CN=localhost"
openssl x509 -req -in "$CERTS_PATH/client.csr" -CA "$CERTS_PATH/ca-intermediate-client.crt" -CAkey "$CERTS_PATH/ca-intermediate-client.key" -CAcreateserial -out "$CERTS_PATH/client.crt" -days 3650 -sha256

# CA bundle
cat "$CERTS_PATH/server.crt" "$CERTS_PATH/ca-intermediate-server.crt" "$CERTS_PATH/ca-intermediate-client.crt" "$CERTS_PATH/ca-root.crt" > "$CERTS_PATH/server-full-chain.crt"
cat "$CERTS_PATH/client.crt" "$CERTS_PATH/ca-intermediate-server.crt" "$CERTS_PATH/ca-intermediate-client.crt" "$CERTS_PATH/ca-root.crt" > "$CERTS_PATH/client-full-chain.crt"
# Also generate a p12 bundle for easy import into browsers
openssl pkcs12 -export -inkey "$CERTS_PATH/client.key" -in "$CERTS_PATH/client.crt" -certfile "$CERTS_PATH/ca-intermediate-client.crt" -out "$CERTS_PATH/client.p12" -name "opnVote client cert" -password pass:opnVote

# Validate Bundles
openssl verify -CAfile "$CERTS_PATH/server-full-chain.crt" "$CERTS_PATH/server.crt"
openssl verify -CAfile "$CERTS_PATH/client-full-chain.crt" "$CERTS_PATH/client.crt"
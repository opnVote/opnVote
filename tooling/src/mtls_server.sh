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

# Simple openssl ca server to test the mtls chain generated with generate_mtls_chain.sh
# Use in conjunction with mtls_client.sh

CERTS_PATH="../../resources/certs"

openssl s_server -accept 8443 -cert "$CERTS_PATH/server.crt" -key "$CERTS_PATH/server.key" -CAfile "$CERTS_PATH/server-full-chain.crt" -Verify 2
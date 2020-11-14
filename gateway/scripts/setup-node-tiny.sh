#!/bin/bash

echo "This script clones the recent version of substrate-node-template and connects EscrowGateway & EscrowGatewayBalance"

git clone https://github.com/substrate-developer-hub/substrate-node-template demo-runtime

cd demo-runtime

cargo build

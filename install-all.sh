#!/bin/bash

git submodule update

cd gateway || exit

echo -e "\033[0;34mBuilding escrow pallets..."
cd pallet-escrow-gateway/escrow-engine && cargo build && cd ../..
cd pallet-escrow-gateway/escrow-engine/versatile-wasm && cargo build && cd ../../..
cd pallet-escrow-gateway/escrow-engine/escrow-contracts-wrapper && cargo build && cd ../../..
cd pallet-escrow-gateway/runtime-gateway && cargo build || (cargo update && cargo build) || exit && cd ../..
cd pallet-escrow-gateway/contracts-gateway && cargo build || (cargo update && cargo build) || exit && cd ../..

echo -e "\033[0;32mTesting escrow pallets..."
cd pallet-escrow-gateway/escrow-engine && cargo test; cd ../..
cd pallet-escrow-gateway/runtime-gateway && cargo test; cd ../..
cd pallet-escrow-gateway/contracts-gatewa && cargo test; cd ../..

echo -e "\033[0;34mBuilding runtimes with escrow pallets..."
echo -e "\033[0;34mBuilding node-tiny..."
cd node-tiny && cargo build || (cargo update && cargo build) || exit && cd ..
echo -e "\033[0;34mBuilding node-full..."
cd node-full/substrate && cargo build || (cargo update && cargo build) || exit && cd ../..

echo -e "\033[0;34mInstalling integration tests (JS)..."
cd test-integration && npm install || exit && cd ..
echo -e "\033[0;32mSo far so good. I will now run integration tests for both node-tiny & node-full by spinning up the nodes for 1 minute, executing test, exiting the node"
chmod +x run-node-full.sh
chmod +x run-node-tiny.sh
npm install -g ttab

ttab -w -a iTerm exec ./run-node-tiny.sh
sleep 5
cd test-integration && npm run test:tiny && cd ..
sleep 60
pkill -f node-tiny

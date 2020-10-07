#!/bin/bash

git submodule update

cd gateway || exit

echo -e "\033[0;34mBuilding escrow pallets..."
cd pallet-escrow-gateway/escrow-engine && cargo build && cd ../..
cd pallet-escrow-gateway/balances && cargo update && cargo build && cd ../..
cd pallet-escrow-gateway && cargo update && cargo build && cd ..

echo -e "\033[0;32mTesting escrow pallets..."
cd pallet-escrow-gateway/balances && cargo test && cd ../..
cd pallet-escrow-gateway && cargo test && cd ..

echo -e "\033[0;34mBuilding runtimes with escrow pallets..."
echo -e "\033[0;34mBuilding node-tiny..."
cd node-tiny && cargo build && cd ..
echo -e "\033[0;34mBuilding node-full..."
cd node-full/substrate && cargo build && cd ../..

echo -e "\033[0;34mInstalling integration tests (JS)..."
cd test-integration && npm install && cd ..
echo -e "\033[0;32mSo far so good. I will now run integration tests for both node-tiny & node-full by spinning up the nodes for 1 minute, executing test, exiting the node"
chmod +x run-node-full.sh
chmod +x run-node-tiny.sh
npm install -g ttab

ttab -w -a iTerm exec ./run-node-tiny.sh
sleep 5
cd test-integration && npm run test:tiny && cd ..
sleep 60
pkill -f node-tiny

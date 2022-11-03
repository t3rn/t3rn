#!/bin/bash

set -x

PARACHAIN_NAME=${1:-t3rn}

echo "building artifacts for: $PARACHAIN_NAME"

ROOT_DIR=$(git rev-parse --show-toplevel)

cd "${ROOT_DIR}"/node/"${PARACHAIN_NAME}"-parachain && cargo build --release --locked || cd ..

mkdir -p "${ROOT_DIR}"/target/release/specs

./"$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain polkadot > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json
./"$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain polkadot --raw > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json
./"$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator export-genesis-wasm --chain "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json --raw > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".wasm

cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json
cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json
cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".wasm "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".wasm

rm -rf "${ROOT_DIR}"/target/release/specs

echo "successfully generated artifacts (.json, raw.json, .wasm) for: $PARACHAIN_NAME to ${ROOT_DIR}/target/release/specs"

#!/bin/bash -e
ROOT_DIR=$(git rev-parse --show-toplevel)

PARACHAIN_NAME=${1:-t3rn}
RELAYCHAIN_NAME=${2:-polkadot}

echo "building artifacts for: $PARACHAIN_NAME at $RELAYCHAIN_NAME"

COLLATOR_BIN=$ROOT_DIR/target/release/${PARACHAIN_NAME}-collator
SPECS_DIR=${ROOT_DIR}/specs

mkdir -p $SPECS_DIR

$COLLATOR_BIN build-spec --chain $RELAYCHAIN_NAME > $SPECS_DIR/${PARACHAIN_NAME}.json
$COLLATOR_BIN build-spec --chain $RELAYCHAIN_NAME --raw > $SPECS_DIR/${PARACHAIN_NAME}.raw.json
# We use WASM that is attached to a release so this step is not needed anymore
# $COLLATOR_BIN export-genesis-wasm --chain $SPECS_DIR/${PARACHAIN_NAME}.raw.json > $SPECS_DIR/${PARACHAIN_NAME}.wasm

rm -rf "${ROOT_DIR}"/target/release/specs

echo "successfully generated artifacts (.json, raw.json) for: ${PARACHAIN_NAME} to ${ROOT_DIR}/target/release/specs"
echo "$PARACHAIN_NAME.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json)"
echo "$PARACHAIN_NAME.raw.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json)"

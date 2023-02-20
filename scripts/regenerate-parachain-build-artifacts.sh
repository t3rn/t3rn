#!/bin/bash

set -x

PARACHAIN_NAME=${1:-t3rn}
RELAYCHAIN_NAME=${2:-polkadot}

echo "building artifacts for: $PARACHAIN_NAME at $RELAYCHAIN_NAME"

ROOT_DIR=$(git rev-parse --show-toplevel)

echo "removing current binaries for $PARACHAIN_NAME"

rm "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator

cd "${ROOT_DIR}"/node/"${PARACHAIN_NAME}"-parachain && cargo build --release --locked || exit

cd "${ROOT_DIR}" || exit

mkdir -p "${ROOT_DIR}"/target/release/specs

exec
  "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain "$RELAYCHAIN_NAME" > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json
  "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain "$RELAYCHAIN_NAME" --raw > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json

cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json
cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json

rm -rf "${ROOT_DIR}"/target/release/specs

echo "successfully generated artifacts (.json, raw.json) for: ${PARACHAIN_NAME} to ${ROOT_DIR}/target/release/specs"
echo "$PARACHAIN_NAME.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json)"
echo "$PARACHAIN_NAME.raw.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json)"

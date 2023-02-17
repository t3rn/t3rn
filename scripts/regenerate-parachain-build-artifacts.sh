#!/bin/bash -ex

PARACHAIN_NAME=${1:-t3rn}
RELAYCHAIN_NAME=${2:-polkadot}

echo "building artifacts for: $PARACHAIN_NAME / $RELAYCHAIN_NAME"

ROOT_DIR=$(git rev-parse --show-toplevel)

mkdir -p "${ROOT_DIR}"/target/release/specs

exec
  "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain "$RELAYCHAIN_NAME" > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json
  "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator build-spec --chain "$RELAYCHAIN_NAME" --raw > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json
  "$ROOT_DIR"/target/release/"${PARACHAIN_NAME}"-collator export-genesis-wasm --chain "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json > "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".wasm

cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json
cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".raw.json "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json
cp "${ROOT_DIR}"/target/release/specs/"${PARACHAIN_NAME}".wasm "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".wasm

echo "successfully generated artifacts (.json, raw.json, .wasm) for: ${PARACHAIN_NAME} to ${ROOT_DIR}/target/release/specs"
echo "$PARACHAIN_NAME.wasm: $(head -c 64 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".wasm)"
echo "$PARACHAIN_NAME.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".json)"
echo "$PARACHAIN_NAME.raw.json: $(head -c 21 "${ROOT_DIR}"/specs/"${PARACHAIN_NAME}".raw.json)"

if [[ $(git diff --stat) != '' ]]; then
  echo 'Chainspecs are not up to date. Please run `./scripts/regenerate_parachain_build_artifacts.sh` and commit the changes.'
  exit 1
fi


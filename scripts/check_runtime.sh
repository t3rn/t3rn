#/usr/env/bin bash
set -e

# support t1rn t0rn and t3rn
if [ -z "$1" ]; then
    echo "Usage: $0 <t0rn|t1rn|t3rn>"
    exit 1
fi

DIR=$(git rev-parse --show-toplevel)
BIN_DIR=$DIR/bin
PARACHAIN_NAME=$1
POLKADOT_CLI_VERSION="@polkadot/api-cli@0.55.3"

git fetch --all --tags -f || true > /dev/null

case "$1" in
    t0rn*)
        RPC_RELAYCHAIN=wss://rococo-rpc.polkadot.io
        PARACHAIN_ID=3333
        RPC_PARACHAIN="wss://rpc.${PARACHAIN_NAME}.io"
    ;;
    t1rn*)
        RPC_RELAYCHAIN=wss://kusama-rpc.polkadot.io
        PARACHAIN_ID=3334
        RPC_PARACHAIN="wss://rpc.${PARACHAIN_NAME}.io"
    ;;
    t3rn*)
        RPC_RELAYCHAIN=wss://rpc.polkadot.io
        PARACHAIN_ID=3333
        # Outdated RPC endpoint which will be changed after runtime upgrade
        RPC_PARACHAIN="wss://ws.${PARACHAIN_NAME}.io"
    ;;
    *)
esac

BLOCK_HASH=$(npm exec -- ${POLKADOT_CLI_VERSION} --ws ${RPC_PARACHAIN} rpc.chain.getFinalizedHead | jq -r .getFinalizedHead)
echo "💈 Script started at ${BLOCK_HASH} block in ${PARACHAIN_NAME} chain"

# Fetch WASM
case "$1" in
    t0rn|t1rn)
        TAG=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
    ;;
    t3rn)
        TAG=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
    ;;
    *)
esac
WASM_BINARY=$DIR/${PARACHAIN_NAME}-parachain-runtime-${TAG}.compact.compressed.wasm

URL=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/${TAG} | jq -r '.assets[] | select(.name | endswith ("compact.compressed.wasm")).browser_download_url' | grep ${PARACHAIN_NAME}-parachain-runtime)
echo "💾 Downloading WASM $URL"

if [ "$URL" == "" ]; then
    echo "🚨 WASM URL is empty! Releasing in process. Aborting."
    exit 1
fi
curl -s -L -o - "$URL" > $WASM_BINARY 2> /dev/null

# Check if wasm exists
if [[ ! -f $WASM_BINARY ]]; then
    echo "🚨 $WASM_BINARY does not exist!"
    exit 1
fi

WASM_HASH=$(subwasm info --json $WASM_BINARY | jq -r .blake2_256)

while true; do
    WASM_HASH_RELAYCHAIN=$(npm exec -- ${POLKADOT_CLI_VERSION} --ws ${RPC_RELAYCHAIN} query.paras.currentCodeHash ${PARACHAIN_ID} | jq -r .currentCodeHash)
    echo
    echo "🫧 Check WASM artifact..."
    echo "🔢 WASM hash in Github Release: $WASM_HASH"
    echo "🔢 WASM hash on Relaychain: $WASM_HASH_RELAYCHAIN"
    
    if [[ "$WASM_HASH" != "$WASM_HASH_RELAYCHAIN" ]]; then
        echo
        echo "🟡 Parachain $PARACHAIN_NAME is running different version then what is in Github Release."
        echo "🟡 Waiting for runtime upgrade to finish..."
    else
        echo
        echo "✅ Parachain $PARACHAIN_NAME is running latest version"
        exit 0
    fi
    sleep 10
done


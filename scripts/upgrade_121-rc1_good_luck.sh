#!/bin/bash
#!/bin/bash

PUSHED_TAG=v1.2.1-rc1
PARACHAIN_NAME=t0rn
CURRENT_HASH=$(curl -sSfH "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead", "params": [ ] }' https://rpc.t0rn.io | jq -r .result)
CURRENT_HEIGHT=$(curl -sSfH "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\", \"params\": [\"${CURRENT_HASH}\"] }" https://rpc.t0rn.io | jq -r .result.number)
UPGRADE_AT_HEIGHT=$(($CURRENT_HEIGHT + 0x64))

echo $CURRENT_HEIGHT $UPGRADE_AT_HEIGHT

#./scripts/upgrade-runtime-unsafe.sh ${{secrets.RUNTIME_UPGRADE_SEED}} wss://ws.t0rn.io https://rpc.t0rn.io ${{env.PUSHED_TAG}} ${UPGRADE_AT_HEIGHT} ${{env.PARACHAIN_NAME}}
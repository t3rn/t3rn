set -euo pipefail

ALICE=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

salt=0x0000000000000000000000000000000000000000000000000000000000000003
gas_limit=3000000000
storage_deposit_limit=100000000000000

flipper_contract_path="./flipper/target/ink/flipper.contract"
flipper_ctor=$(jq -r .V3.spec.constructors[0].selector "$flipper_contract_path")
flipper_wasm=$(jq -r .source.wasm "$flipper_contract_path")
printf "%s null" $flipper_wasm > /tmp/flipper.params

karura_contract_path="./karura/target/ink/karura.contract"
karura_ctor=$(jq -r .V3.spec.constructors[0].selector "$karura_contract_path")
karura_wasm=$(jq -r .source.wasm "$karura_contract_path")
printf "%s null" $karura_wasm > /tmp/karura.params

function instantiate() {
  contract_path=$1
  contract_name=$2
  contract_data=$3
  value=$4
  ctor=$5

  hash=$(jq -r .source.hash "$contract_path")
  gas_limit=1000000000
  storage_deposit_limit=100000000000000

  printf \
    "%d %d %d %s %s %s" \
    "$value" \
    $gas_limit \
    $storage_deposit_limit \
    $hash \
    contract_data \
    $salt \
    >/tmp/"$contract_name".params

  polkadot-js-api \
    --ws ws://localhost:9944 \
    --seed //Alice \
    --params /tmp/$contract_name.params \
    tx.contracts.instantiate
}

function instantiate_flipper() {
  contract_name=flipper
  printf \
    "%d %d %d %s %s %s" \
    0 \
    $gas_limit \
    $storage_deposit_limit \
    $flipper_wasm \
    $1 \
    $salt \
    >/tmp/"$contract_name".params

  polkadot-js-api \
    --ws ws://localhost:9944 \
    --seed //Alice \
    --params /tmp/$contract_name.params \
    tx.contracts.instantiateWithCode
}

function call_flipper() {
  contract_name=flipper
  printf \
    "%s %d %d %s %s" \
    $1 \
    0 \
    $gas_limit \
    $storage_deposit_limit \
    "0x485553280400" \
    >/tmp/"$contract_name".params

  polkadot-js-api \
    --ws ws://localhost:9944 \
    --seed //Alice \
    --params /tmp/$contract_name.params \
    tx.contracts.call
}

function instantiate_karura() {
  contract_name=karura
  printf \
    "%d %d %d %s %s %s" \
    0 \
    $gas_limit \
    $storage_deposit_limit \
    0x2ceaa8912064d49ed65370f89943675909c65890f3c5ed89d5c033f4789e13c5 \
    $karura_ctor \
    $salt \
    >/tmp/"$contract_name".params

  polkadot-js-api \
    --ws ws://localhost:9944 \
    --seed //Alice \
    --params /tmp/$contract_name.params \
    tx.contracts.instantiate
}

karura_ss58=$(instantiate_karura | jq -r '.instantiate.events[2].event.data[0] | select( . != null )')

printf \
    "%s %d" \
    $karura_ss58 \
    100000000000000 \
    > /tmp/balance.params

polkadot-js-api \
    --ws ws://localhost:9944 \
    --seed //Alice \
    --params /tmp/balance.params \
    tx.balances.transfer

#karura_hex=$(subkey inspect $karura_ss58 | grep 'Public key (hex)' | sed 's/  Public key (hex):   0x//')
#
#flipper_id=$(instantiate_flipper "${flipper_ctor}$karura_hex" | jq -r '.instantiateWithCode.events[1].event.data[0] | select( . != null )')
#call_flipper $flipper_id
#
#
#printf \
#    "%s %d" \
#    $flipper_id \
#    100000000000000 \
#    > /tmp/balance.params
#
#polkadot-js-api \
#    --ws ws://localhost:9944 \
#    --seed //Alice \
#    --params /tmp/balance.params \
#    tx.balances.transfer

echo "karura ss58 id $karura_ss58"
#echo "flipper ss58 id $flipper_id"

#!/bin/bash

# This script is used to test the maintenance mode of Rococo for t0rn (and possibly t3rn)

POLKADOT_CLI_VERSION="@polkadot/api-cli@0.52.27"
parachain_name="t0rn"
ws_provider="wss://ws.${parachain_name}.io"
http_provider="https://rpc.${parachain_name}.io"
sudo_secret="$1"
dryrun=$(echo "$@" | grep -o dry) || true



# Check maintenance mode before

status_before="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    query.maintenanceMode.maintenanceMode | \
    jq '.maintenanceMode' )"

echo
echo "Maintenance mode status (before): ${status_before}"
echo



# Enter maintenance mode

echo
echo "Entering maintenance mode..."
echo

# Only run if dryrun flag is NOT set
if [[ -z $dryrun ]]; then
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    --sudoUncheckedWeight "100000" \
    --seed "$sudo_secret" \
    tx.maintenaceMode.enterMaintenanceMode

  echo
  echo "✅ Entered maintenance mode!"
  echo
fi



# Test some calls

echo
echo "🤖 Testing some calls (not all, though)..."
echo

echo "Asset Registry..."
assetRegistry="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    tx.assetRegistry.registerInfo 0 )"
echo "Asset Registry: ${assetRegistry:-❌ Not OK, call passed}"

echo "Circuit..."
circuit="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    tx.circuit.onXcmTrigger )"
echo "Circuit: ${circuit:-❌ Not OK, call passed}"

echo "EVM..."
evm="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    tx.evm.claim )"
echo "EVM: ${evm:-❌ Not OK, call passed}"



# Always exit maintenance mode

echo
echo "Done! Exiting maintenance mode..."

npx --yes $POLKADOT_CLI_VERSION \
  --ws $ws_provider \
  --sudoUncheckedWeight "100000" \
  --seed "$sudo_secret" \
  tx.maintenaceMode.resumeNormalOperation

echo
echo "✅ Exited maintenance mode!"
echo



# Check maintenance mode after

status_after="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    query.maintenanceMode.maintenanceMode | \
    jq '.maintenanceMode' )"

echo
echo "Maintenance mode status (after): ${status_after}"
echo
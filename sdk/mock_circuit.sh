#!/bin/sh

if ! command -v cargo-t3rn-contract &> /dev/null; then
  echo -e "\033[0;34mcargo-t3rn-contracts could not be found"
  echo -e "\033[0;34mBuild the latest t3rn compiler from source (./compiler) ..."

  # change rust version to 1.55.0 due to build errors at subxt v0.12 and latest cargo
  rustup default nightly-2021-08-01
  rustup component add rust-src
  rustup target add wasm32-unknown-unknown

  cd ./compiler || exit
  cargo build --features extrinsics || exit

  cd .. || exit
  T3RN_COMPILER_BIN=$(pwd)/compiler/target/debug/cargo-t3rn-contract
else
  echo -e "\033[0;34mUse $(command -v cargo-t3rn-contract)..."
  T3RN_COMPILER_BIN=cargo-t3rn-contract
fi

echo -e "\033[0;34mStart a demo runtime node with runtime + contract gateways hooked on in a separate terminal (using npm ttab)..."
if [ "$(uname)" == "Darwin" ]; then
    TERM_NAME=iTerm
else
    TERM_NAME=gnome-terminal
fi
npm install -g ttab
cd ../gateway || exit
# Pre-install in the same terminal, can take time if build isn't ready.
# # cd demo-runtime || exit
cargo build || exit

# Run demo-node in a separate terminal; should be fast as the build is ready.
ttab -w -a $TERM_NAME exec ./run-node-tiny.sh
cd ../sdk || exit
sleep 8

# change rust version to latest
rustup default nightly
rustup component add rust-src
rustup target add wasm32-unknown-unknown

echo -e "\033[0;34mBuilding demo contract components using composable-build..."
cd ./examples/haphazard_demo_storage || exit
$T3RN_COMPILER_BIN contract composable-build

echo -e "\033[0;34mDeploy contract components using composable-deploy..."
DEPLOY_OUTPUT=$($T3RN_COMPILER_BIN contract composable-deploy --suri //Alice)

REGEX_CODE_HASH='0x([0-9a-f]{64})'
REGEX_INST_ACCOUNT='Contract account: ([0-9a-f]{64})'
REGEX_FLIP_RESULT='contract result: (\[1, 0\]|\[0, 0\])'
if [[ $DEPLOY_OUTPUT =~ $REGEX_CODE_HASH ]]; then
    for key in "${!BASH_REMATCH[@]}"; do
        if [ "$key" -gt "0" ]; then
          echo -e "\033[0;34mSuccessfully deployed contract with on-chain hash code: ${BASH_REMATCH[$key]}"
          echo -e "\033[0;34mInstantiate deployed contract components..."
          INSTANTIATED_CONTRACT_OUTPUT=$($T3RN_COMPILER_BIN contract instantiate --code-hash "${BASH_REMATCH[$key]}" --suri //Alice --data d183512b01 --gas 3875000000 --endowment 10000000)
          echo "$INSTANTIATED_CONTRACT_OUTPUT"
          if [[ $INSTANTIATED_CONTRACT_OUTPUT =~ $REGEX_INST_ACCOUNT ]]; then
             CONTRACT_ACCOUNT="${BASH_REMATCH[1]}"
              echo -e "\033[0;34mSuccessfully instantiated contract at address: $CONTRACT_ACCOUNT"
             # Cool, flipper contract is there. From this point a "real" logic starts.
             # The idea behind this demo is to:
             #  1. Execute runtime demo "EXEC" phase first via Runtime Gateway.
             #  2. Flip via regular contract call.
             #  3. If flip.value = true, execute "COMMIT" phase for runtime demo via Runtime Gateway
             #  4. If flip.value = false, execute "REVERT" phase for runtime demo via Runtime Gateway
             echo -e "\033[0;36mExecuting demo contract via runtime gateway..."
             EXEC_DEMO_OUTPUT=$($T3RN_COMPILER_BIN contract call-runtime-gateway --phase 0 --data 16000000 --suri //Alice --target //Bob --requester //Bob ./target/runtime_demo_storage/runtime_demo_storage.wasm)
             echo -e "\033[0;36mSuccessfully executed EXEC phase of demo contract via runtime gateway: "
             echo "$EXEC_DEMO_OUTPUT"
             CALL_FLIP_OUTPUT=$($T3RN_COMPILER_BIN contract call-contract --data c096a5f3 --suri //Alice --target "$CONTRACT_ACCOUNT")
             if [[ $CALL_FLIP_OUTPUT =~ $REGEX_FLIP_RESULT ]]; then
               echo -e "\033[0;32mCoin flip result successfully recognized: ${BASH_REMATCH[1]}"
               if [[ "${BASH_REMATCH[1]}" = '[0, 0]' ]]; then
                  echo -e "\033[0;32mCoin flip resulted with FALSE! Commit demo storage execution via runtime gateway."
                  COMMIT_DEMO_OUTPUT=$($T3RN_COMPILER_BIN contract call-runtime-gateway --phase 1 --data 16000000 --suri //Alice --target //Bob --requester //Bob ./target/runtime_demo_storage/runtime_demo_storage.wasm)
                  echo -e "\033[0;32mSuccessful COMMIT phase of demo contract via runtime gateway"
               fi
               if [[ "${BASH_REMATCH[1]}" = '[1, 0]' ]]; then
                  echo -e "\033[0;33mCoin flip resulted with TRUE. Revert demo storage execution via runtime gateway."
                  REVERT_DEMO_OUTPUT=$($T3RN_COMPILER_BIN contract call-runtime-gateway --phase 2 --data 16000000 --suri //Alice --target //Bob --requester //Bob ./target/runtime_demo_storage/runtime_demo_storage.wasm)
                  echo -e "\033[0;33mSuccessful REVERT phase of demo contract via runtime gateway"
               fi
             else
                echo -e "\033[0;31mError: Can't recognize flip result: $CALL_FLIP_OUTPUT"
             fi
          fi
        fi
    done
fi

echo -e "\033[0;34mKill the demo runtime node..."
pkill -f demo-runtime

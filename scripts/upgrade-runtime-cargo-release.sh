#!/bin/bash -e

echo "Replacing version in docs with $NEW_VERSION"

sed -i.bak -E "s/v[0-9]+\\.[0-9]\\.[0-9]-rc\\.[0-9]/v${NEW_VERSION}/g" $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md
rm $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md.bak

echo "Increment versions for ${PARACHAIN_NAME} runtime"
sed -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
rm $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
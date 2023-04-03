#!/bin/bash -e

echo "Replacing version in docs with $NEW_VERSION"
sed -i.bak -E "s/v[0-9]+\\.[0-9]+\\.[0-9]+-rc\\.[0-9]+/v${NEW_VERSION}/g" $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md
rm $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md.bak

if git diff --quiet $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs; then
  echo "Increment versions for ${PARACHAIN_NAME} runtime"
  sed -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
  sed -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
  sed -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
  sed -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
  rm $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
else
  echo "Because cargo release pre-release-hooks are run per target we need to make sure that versions are incremented only once"
fi

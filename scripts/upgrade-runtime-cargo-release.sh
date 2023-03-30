#!/bin/bash -e

echo "Replacing version in docs with $NEW_VERSION"

sed -i.bak -E "s/v[0-9]+\\.[0-9]\\.[0-9]-rc\\.[0-9]/v${NEW_VERSION}/g" $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md
rm $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md.bak

echo "Increment versions for ${PARACHAIN_NAME} runtime"
gsed -i.bak -r 's/(impl_version:)(.*)/echo \1 $(echo "\2 + 1"\|bc)/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs 
gsed -i.bak -r 's/(transaction_version:)(.*)/echo \1 $(echo "\2 + 1"\|bc)/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs 
gsed -i.bak -r 's/(authoring_version:)(.*)/echo \1 $(echo "\2 + 1"\|bc)/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs 
gsed -i.bak -r 's/(specVersion:)(.*)/echo \1 $(echo "\2 + 1"\|bc)/ge' $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs 
rm $GITHUB_WORKSPACE/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
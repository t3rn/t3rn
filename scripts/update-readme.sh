#!/bin/bash -e

echo "Replacing version in docs with $NEW_VERSION"

sed -i.bak -E "s/v[0-9]+\\.[0-9]\\.[0-9]-rc\\.[0-9]/v${NEW_VERSION}/g" $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md
rm $GITHUB_WORKSPACE/docs/main/docs/collator/testnet/testnet-collator.md.bak
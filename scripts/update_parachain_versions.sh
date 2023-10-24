#!/bin/bash -e
DIR=$(git rev-parse --show-toplevel)
cd $DIR

echo "Increment versions for ${1} runtime"
sed -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${1}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${1}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${1}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${1}-parachain/src/lib.rs

echo
echo Incremented versions:
cat runtime/${1}-parachain/src/lib.rs | grep version

echo

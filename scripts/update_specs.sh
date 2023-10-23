#!/bin/bash -e

echo "Increment versions for ${PARACHAIN_NAME} runtime"
sed -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
sed -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' runtime/${PARACHAIN_NAME}-parachain/src/lib.rs

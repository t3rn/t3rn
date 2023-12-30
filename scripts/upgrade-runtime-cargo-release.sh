#!/bin/bash -e
DIR=$(git rev-parse --show-toplevel)
cd $DIR

SED=sed
if [[ $(uname -s 2>/dev/null || echo not) == "Darwin" ]]; then
    SED=gsed
fi

echo "Replacing version in docs with $NEW_VERSION"
$SED -i.bak -E "s/v[0-9]+\\.[0-9]+\\.[0-9]+-rc\\.[0-9]+/v${NEW_VERSION}/g" $DIR/docs/main/docs/collator/testnet/testnet-collator.md
rm $DIR/docs/main/docs/collator/testnet/testnet-collator.md.bak

PARACHAIN_NAME=t0rn
if git diff --quiet $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs; then
    echo "Increment versions for ${PARACHAIN_NAME} runtime"
    $SED -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    rm $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
else
    echo "Because cargo release pre-release-hooks are run per target we need to make sure that versions are incremented only once"
fi

PARACHAIN_NAME=t1rn
if git diff --quiet $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs; then
    echo "Increment versions for ${PARACHAIN_NAME} runtime"
    $SED -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    rm $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
else
    echo "Because cargo release pre-release-hooks are run per target we need to make sure that versions are incremented only once"
fi

PARACHAIN_NAME=t2rn
if git diff --quiet $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs; then
    echo "Increment versions for ${PARACHAIN_NAME} runtime"
    $SED -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    $SED -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
    rm $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
else
    echo "Because cargo release pre-release-hooks are run per target we need to make sure that versions are incremented only once"
fi

# PARACHAIN_NAME=t7rn
# if git diff --quiet $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs; then
#     echo "Increment versions for ${PARACHAIN_NAME} runtime"
#     $SED -i.bak -r 's/([[:blank:]]+impl_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
#     $SED -i.bak -r 's/([[:blank:]]+transaction_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
#     $SED -i.bak -r 's/([[:blank:]]+authoring_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
#     $SED -i.bak -r 's/([[:blank:]]+spec_version: )([0-9]+)(.*)/echo "\1$(echo \"\2 + 1\"|bc)\3"/e' $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs
#     rm $DIR/runtime/${PARACHAIN_NAME}-parachain/src/lib.rs.bak
# else
#     echo "Because cargo release pre-release-hooks are run per target we need to make sure that versions are incremented only once"
# fi

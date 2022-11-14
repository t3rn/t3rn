#!/bin/bash
new_version=$(($2+1))

sed -i '' "s/$3: $2/$3: $new_version/g" "$(git rev-parse --show-toplevel)"/runtime/"$1"-parachain/src/lib.rs

#!/bin/bash
new_version=$(($2+1))
bin_dir=../../bin

case "$1" in
  t0rn*)
    echo "=========== T0RN ==========="
    tags_list=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 2)
    echo Tags:
    echo "$tags_list"

    SAVEIFS=$IFS   # Save current IFS (Internal Field Separator)
    IFS=$'\n' 
    tags_list=($tags_list) # split to array
    old_version=${tags_list[1]}
    echo Old version: "$old_version"
    new_version=${tags_list[0]}
    echo New version: "$new_version"

    tag_version="$old_version"
    echo Version: "$tag_version"

    ./download_collator.sh "$old_version" "-old"
    ./download_latest_wasm_blob.sh "$new_version"
    ;;
  t3rn*)
    echo "=========== T3RN ==========="
    tags_list=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 2)
    echo Tags: "$tags_list"

    SAVEIFS=$IFS   # Save current IFS (Internal Field Separator)
    IFS=$'\n' 
    tags_list=($tags_list) # split to array
    old_version=${tags_list[1]}
    echo Old version: "$old_version"
    new_version=${tags_list[0]}
    echo New version: "$new_version"

    tag_version="$old_version"
    echo Version: "$tag_version"

    ./download_collator.sh "$old_version" "-old"
    ./download_latest_wasm_blob.sh "$new_version"
    ;;
  *)        
  exit 1;;
esac

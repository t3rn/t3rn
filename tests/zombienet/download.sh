#!/usr/bin/env bash
bin_dir=../../bin

echo
echo ⬇️ Downloading binaries for $1
echo

# Make sure we have all recent tags
git fetch --all --tags -f || true > /dev/null

case "$1" in
    t0rn*)
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
        echo New version: ${new_version}
        
        ./download_previous_collator.sh t0rn
        ./download_latest_wasm_blob.sh t0rn ${new_version}
    ;;
    t3rn*)
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
        echo Tags: ${tags_list}
        echo New version: ${new_version} t3rn
        
        ./download_previous_collator.sh t3rn
        ./download_latest_wasm_blob.sh t3rn ${new_version}
    ;;
    *)
    exit 1;;
esac

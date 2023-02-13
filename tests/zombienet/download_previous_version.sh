#!/bin/bash -x
new_version=$(($2+1))

case "$1" in
  t0rn*)
      echo =========== T0RN OLD COLLATOR ===========; \
      export tag_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 2 | tail -n 1)
      echo Version: "$tag_version"

      ./download_collator.sh "$tag_version" 
    ;;
  t3rn*)
    echo =========== T3RN OLD COLLATOR ===========; \
      export tag_version=$(version=git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 2 | tail -n 1)
      echo Version: "$tag_version"

      ./download_collator.sh "$tag_version" 
    ;;
  *)        exit 1;;
esac

#!/bin/bash
new_version=$(($2+1))

case "$1" in
  t0rn*)
      echo =========== T0RN OLD COLLATOR ===========; \
      export tag_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
      echo Version: "$tag_version"

      export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$tag_version" | jq -r '.assets[0].browser_download_url')
      echo Url: $url
      
      [ "$url" ] || exit 1
      curl -L -o - "$url" | gunzip -c > "$bin_dir"/collator-old && chmod +x "$bin_dir"/collator-old
    ;;
  t3rn*)
    echo =========== T3RN OLD COLLATOR ===========; \
      export tag_version=$(version=git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
      echo Version: "$tag_version"

      export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$tag_version" | jq -r '.assets[0].browser_download_url')
      echo Url: $url
      
      [ "$url" ] || exit 1
      curl -L -o - "$url" | gunzip -c > "$bin_dir"/collator-old && chmod +x "$bin_dir"/collator-old
    ;;
  *)        exit 1;;
esac

repo_owner="t3rn"
repo_name="t3rn"
releases=$(curl -s "https://api.github.com/repos/$repo_owner/$repo_name/releases")

parachain=$1

# This gets all the releases and finds only the releases that include the parachain binary
# It then prints the tag name and the download url for the binary

# If no parachain is specified, it will print all the releases and their binaries
# If a parachain is specified, it will print all the releases that include that parachain binary

only_latest_release=false

if [[ $2 == "--latest" ]]; then
    only_latest_release=true
fi

# If the second argument is --upgrade, it will skip the first asset and get the second asset
# This is useful for getting the previous binary so we can upgrade it with the latest wasm
skip_first_asset=false
has_skipped=false
if [[ $2 == "--upgrade" ]]; then
    skip_first_asset=true
fi

for release in $(echo "${releases}" | jq -r '.[] | @base64'); do
    release_json=$(echo "${release}" | base64 --decode | jq -r '.')
    tag=$(echo "${release_json}" | jq -r '.tag_name')
    
    assets=$(echo "${release_json}" | jq -r '.assets')

    for asset in $(echo "${assets}" | jq -r '.[] | @base64'); do
        asset_json=$(echo "${asset}" | base64 --decode | jq -r '.')
        asset_name=$(echo "${asset_json}" | jq -r '.name')

        if [[ "$asset_name" == *unknown-linux-gnu.gz ]]; then
            if [[ "$asset_name" == *"$parachain"* ]]; then
                echo $tag: $(echo "${asset_json}" | jq -r '.browser_download_url')
                
                if [[ $has_skipped == true ]]; then
                    exit 0
                fi
                if [[ $skip_first_asset == true ]]; then
                    has_skipped=true
                fi
                if [[ $only_latest_release == true ]]; then
                    exit 0
                fi
            fi
        fi
    done
done

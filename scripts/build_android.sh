#!/bin/bash

ENTITLEMNT_PATH=src-tauri/gen/android/app/build.gradle.kts
if ! grep -q "keystore.properties" $ENTITLEMNT_PATH; then
    # should contain:
    # https://tauri.app/distribute/sign/android/#configure-the-signing-key
    echo "ERROR: keystore.properties not found in build.gradle.kts file!"
    exit 1
fi

pnpm tauri android build

# Set up a service account for the Play Store API
# https://github.com/chippmann/androidpublisher/blob/master/CONFIGURATION_OF_GOOGLE_SERVICE_ACCOUNT.md
# Put JSON key in environment variable GOOGLE_SERVICE_ACCOUNT
script_dir=$(dirname "$0")
uv run $script_dir/playstore_upload.py

VERSION=$(jq -r '.version' package.json)

gh release create "v$VERSION" "$script_dir/../src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk"

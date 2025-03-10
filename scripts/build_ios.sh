#!/bin/bash

ENTITLEMNT_PATH=src-tauri/gen/apple/fds-scanner_iOS/fds-scanner_iOS.entitlements
if ! grep -q "webcredentials" $ENTITLEMNT_PATH; then
    # should contain:
    # <string>webcredentials:fragdenstaat.de</string>
    echo "ERROR: webcredentials not found in entitlements file!"
    exit 1
fi

# Set vars from .env file
set -a && source .env && set +a
# Unset vars that would make tauri build
# go into automatic signing mode, we want manual signing
unset APPLE_API_KEY_PATH
unset APPLE_API_KEY
unset APPLE_API_KEY_ID
unset APPLE_API_ISSUER
set -e

# export TAURI_CLI_VERBOSITY=3
pnpm tauri ios build --export-method app-store-connect

# The alt tool needs the apple api credentials for upload again
set -a && source .env && set +a
xcrun altool --upload-app --type ios --file "src-tauri/gen/apple/build/arm64/FDS Scanner.ipa" --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER

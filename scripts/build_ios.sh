#!/bin/bash

. .env
set -ex


pnpm tauri icon public/fds-scanner.png --ios-color "#fff"
pnpm tauri ios build --export-method app-store-connect
xcrun altool --upload-app --type ios --file src-tauri/gen/apple/build/arm64/fds-scanner.ipa --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER
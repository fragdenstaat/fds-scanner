# FragDenStaat Scanner

This is a Tauri 2.0 mobile application using Vue and Ionic Framework.

## Project setup

See [Tauri Prerequesites](https://tauri.app/start/prerequisites/)

Set

## Distribution

See `.env-exmaple` for the required environment variables.

### iOS



```bash
pnpm tauri ios init
script/build-ios.sh
```

### Android

pnpm tauri android init


```bash
pnpm tauri android init
pnpm tauri icon public/fds-scanner.png --ios-color "#fff"
# Follow instructions to configure signing
# https://tauri.app/distribute/sign/android/#configure-the-signing-key
pnpm tauri android build
```

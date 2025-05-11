# Tauri + React + Typescript

# Error Description:
On macOS, Webview windows created in a Tauri v2 application with .transparent(true) render correctly (fully transparent backgrounds) when the app is launched via tauri dev, but those same windows lose transparency and appear as solid white rectangles after the project is packaged with tauri build --bundles dmg and run from the resulting DMG‑installed .app. 

The issue occurs even when "macOSPrivateApi": true is set in tauri.conf.json and the build is forced with TAURI_PRIVATE_API=1 and the tauri/macos-private-api feature. 

It seems like the transparency flag survives in the development build but is dropped or ignored during the bundling/signing phase, leading to inconsistent window behavior between dev and release builds.

# To recreate:

To recreate the error simply:
1. Run 'bun install' to install dependencies
2. Run 'bun run tauri dev', then 'open window' to see the transparent window
3. Run 'bun tauri build --bundles dmg' to build dmg, drag to applications, then run the app to see opaque, white window
## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
# test

Local
<img width="459" alt="Screenshot 2025-05-11 at 10 39 39 AM" src="https://github.com/user-attachments/assets/ea12c021-ae47-4677-9d6a-1541d757032b" />

Bundled DMG
<img width="451" alt="Screenshot 2025-05-11 at 10 39 58 AM" src="https://github.com/user-attachments/assets/d839e277-ec4b-4d8a-9b12-b5d67ba645d1" />

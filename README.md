# PDF Maker

A desktop app (Windows, built with [Tauri](https://tauri.app)) for turning photos, `.txt`/`.md` files, and `.docx` files into a PDF. Combine everything into one PDF or export one per file, reorder/rotate/crop photos, switch color/grayscale, and keep a saved library of PDFs on your machine.

This is a Tauri port of an original Capacitor/iOS build of the same app — the UI and PDF logic are unchanged; only the native integration layer was swapped (see [What changed](#what-changed-from-the-ios-version)).

## Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Windows build tools: the "Desktop development with C++" workload from [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), and [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (preinstalled on Windows 10/11 in most cases)

Full platform-by-platform setup: <https://v2.tauri.app/start/prerequisites/>

## Getting started

```bash
npm install
npm run dev      # launches the app with hot reload
```

## Building an installer locally

```bash
npm run build
```

Output lands in `src-tauri/target/release/bundle/` (`.msi` and `.exe`/NSIS installers).

## Cutting a GitHub release

Push a version tag and the `.github/workflows/release.yml` workflow builds a Windows installer and attaches it as a **draft** GitHub Release for you to review and publish:

```bash
git tag v0.1.0
git push origin v0.1.0
```

You can also trigger it manually from the Actions tab (`workflow_dispatch`).

## Project layout

```
frontend/            static HTML/CSS/JS frontend (no build step / bundler)
  index.html
  vendor/             jsPDF + mammoth.js, vendored locally (no CDN dependency)
src-tauri/            Rust/Tauri shell
  src/main.rs
  tauri.conf.json
  capabilities/        permission grants for the dialog/fs/opener plugins
  icons/                app icon (placeholder, see below)
.github/workflows/    Windows release build
```

## What changed from the iOS version

The original file was built for Capacitor and used native iOS plugins (Camera, Filesystem, Share) with browser fallbacks. For this desktop build:

- Capacitor script includes and the Camera/Filesystem/Share native plugin code paths were removed — the app now just uses the file picker and drag-and-drop, same as its own browser fallback did.
- "Download" and "Share" now use `@tauri-apps/plugin-dialog` and `@tauri-apps/plugin-fs` for a native Save As dialog, and `@tauri-apps/plugin-opener`'s `revealItemInDir` to show the saved PDF in File Explorer (the desktop equivalent of a mobile share sheet). These are wired up through `window.__TAURI__` (`app.withGlobalTauri` in `tauri.conf.json`), so the frontend still needs no bundler.
- Running the same `index.html` in a plain browser tab still works — `isTauri` is `false` there, so it falls back to a normal `<a download>` link and the Web Share API, same as before.
- Everything else (PDF generation via jsPDF, `.docx` parsing via mammoth.js, cropping, the saved-PDF library in IndexedDB, dark mode) is untouched.

## App icon

`src-tauri/icons/` contains a generated placeholder icon (not a designed logo). To replace it: drop a 1024x1024 PNG at `src-tauri/icons/icon.png` and run `npx tauri icon src-tauri/icons/icon.png` to regenerate the full set, then update the `icon` array in `tauri.conf.json` if you add platforms beyond Windows (e.g. `icon.icns` for macOS).

## Note on this scaffold

This repository was assembled in an environment without a Rust toolchain, so the Cargo/Tauri configuration hasn't been compiled or run locally — it follows the standard Tauri v2 project layout, but run `npm run dev` first and check `src-tauri/capabilities/default.json` if you hit a permission error on the save dialog (the `fs:scope` there is intentionally broad, since the save path is always a location the user picked via the native dialog).

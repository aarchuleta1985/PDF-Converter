# PDF Maker

A simple desktop app for building and converting PDFs, wrapped as a Windows
desktop app with [Electron](https://www.electronjs.org/). The UI itself
(`src/index.html`) uses [jsPDF](https://github.com/parallax/jsPDF) and
[Mammoth.js](https://github.com/mwilliamson/mammoth.js) entirely client-side
— no server, no network calls.

## Project structure

```
pdf-maker/
├── src/
│   ├── index.html          # the app UI
│   └── vendor/
│       ├── jspdf.umd.min.js
│       └── mammoth.browser.min.js
├── build/
│   ├── icon.ico             # Windows app icon
│   └── icon.png
├── main.js                  # Electron main process
├── preload.js                # Electron preload script
├── package.json              # app + electron-builder config
└── .github/workflows/build.yml  # CI: builds .exe/.msi installers
```

## Run it locally

```bash
npm install
npm start
```

## Build a Windows installer

Windows installers (NSIS `.exe` and `.msi`) must be built on Windows, or on
CI. Locally on Windows:

```bash
npm install
npm run dist:win     # builds both .exe and .msi into dist/
npm run dist:exe     # NSIS .exe installer only
npm run dist:msi     # MSI installer only
```

The installers are written to `dist/`.

### Automatic builds via GitHub Actions

Push this repo to GitHub and the workflow in
`.github/workflows/build.yml` builds both installers on a
`windows-latest` runner. Grab the results from:

- **Actions tab → latest run → Artifacts**, for every push, or
- **Releases**, when you push a tag like `v1.0.0` (the workflow attaches
  the `.exe` and `.msi` to the release automatically).

## Publish to GitHub

```bash
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/<your-username>/pdf-maker.git
git push -u origin main
```

Then tag a release to trigger an installer build:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## License

MIT — see [LICENSE](LICENSE).

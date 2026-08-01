// Preload script runs in an isolated context with access to Node APIs.
// The PDF Maker UI is fully self-contained (jsPDF + Mammoth.js), so no
// extra bridge APIs are needed yet. Add contextBridge.exposeInMainWorld()
// calls here if the app needs native file-system access later.

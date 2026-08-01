fn main() {
    // Tell Cargo to recompile whenever anything in the frontend changes.
    // Without this, the frontend assets get baked into the binary via
    // generate_context!() at compile time, but Cargo has no way of knowing
    // frontend/index.html (or vendor/*.js) changed — so on a cached build
    // (e.g. CI's rust-cache reusing target/), it can silently keep shipping
    // a stale copy of the frontend even after the source file is fixed and
    // the version number is bumped.
    println!("cargo:rerun-if-changed=../frontend");
    tauri_build::build()
}

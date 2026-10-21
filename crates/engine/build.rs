//! Optional build input: with `CRAFT_FONTS_DIR=<storytold/craft-fonts checkout>`, embed the
//! document fonts as `CRAFT_FONTS` (see src/fonts.rs): the CJK families that text drawn into
//! photos falls back on after Inter (export watermarks, slideshow overlays, print captions).
//! Unset, `CRAFT_FONTS` is empty. craft-fonts is never a Cargo dependency, and this script only
//! reads a local directory (no network). The interface's faces are embedded by
//! `crates/ui-egui/build.rs`; the CLI gets only this table.
//!
//! wasm32 embeds BIZ UDPGothic Regular (~4.5 MB) alone, to keep the `.wasm` within the web size
//! budget (Cloudflare serves at most 25 MiB per file).
use std::path::{Path, PathBuf};

/// The document families, as craft-fonts' manifest names them. Mincho faces draw first
/// (`text::faces`).
const DOCUMENT_FAMILIES: &[&str] = &["Shippori Mincho", "BIZ UDMincho", "BIZ UDPGothic", "Noto Sans CJK SC"];

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let wasm = craft_fonts_build::target_is_wasm();
    let mut fonts = craft_fonts_build::Embed::documents(DOCUMENT_FAMILIES)
        .filter(move |face| !wasm || (face.family == "BIZ UDPGothic" && face.style == "Regular"));
    if let Some(dir) = std::env::var_os("CRAFT_FONTS_DIR").filter(|dir| !dir.is_empty()) {
        fonts = fonts.dir(resolve(PathBuf::from(dir)));
    }
    match fonts.write("craft_fonts.rs") {
        // Without them the app still runs, but Japanese and Chinese watermarks draw boxes.
        Ok(faces) if faces.is_empty() => println!(
            "cargo::warning=no craft-fonts document faces embedded: Chinese and Japanese watermark text will render as \
             empty boxes. Clone https://github.com/storytold/craft-fonts next to this repo and rebuild with \
             CRAFT_FONTS_DIR=../craft-fonts"
        ),
        Ok(_) => {}
        Err(e) => println!("cargo::error={e}"),
    }
}

/// Resolve a relative `CRAFT_FONTS_DIR` against both roots a caller might mean: a build script's
/// cwd is the *package* root (`crates/engine`), while the `../craft-fonts` the README and
/// craft-fonts' docs use is written relative to the *workspace* root and would otherwise resolve
/// to `crates/craft-fonts`.
fn resolve(dir: PathBuf) -> PathBuf {
    if dir.is_absolute() {
        return dir;
    }
    let package = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let workspace = package.parent().and_then(Path::parent).unwrap_or(package.as_path());
    let candidates = [workspace.join(&dir), package.join(&dir)];
    candidates.iter().find(|candidate| candidate.is_dir()).cloned().unwrap_or_else(|| candidates[0].clone())
}

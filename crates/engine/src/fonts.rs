//! Document fonts from the optional craft-fonts build input (storytold/craft-fonts).
//!
//! Built with `CRAFT_FONTS_DIR=<craft-fonts checkout>`, [`CRAFT_FONTS`] holds the CJK faces that
//! text drawn into photos uses after Inter: export watermarks, slideshow overlays and print
//! captions (the families are listed in `build.rs`). Otherwise it is empty and that text draws with
//! Inter alone. The interface embeds its own faces (`lightcraft-ui-egui`'s `build.rs`). See
//! craftrules `standards/fonts.md`.

pub use craft_fonts_build::EmbeddedFace;

/// The document faces, in manifest order (empty unless built with `CRAFT_FONTS_DIR`).
pub static CRAFT_FONTS: &[EmbeddedFace] = include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));

/// The faces for any CJK script: what any CJK text needs, regardless of language.
pub fn cjk(fonts: &'static [EmbeddedFace]) -> impl Iterator<Item = &'static EmbeddedFace> {
    fonts.iter().filter(|f| f.is_cjk())
}

/// Mincho (serif) faces, the preferred Japanese faces for document-like text.
pub fn is_mincho(face: &EmbeddedFace) -> bool {
    face.family.contains("Mincho")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn craft_fonts_are_well_formed_or_absent() {
        if CRAFT_FONTS.is_empty() {
            lightcraft_testkit::skip!(Fonts, "built without CRAFT_FONTS_DIR: no craft-fonts to check");
            return;
        }
        for f in CRAFT_FONTS {
            assert!(!f.family.is_empty() && !f.style.is_empty() && !f.scripts.is_empty(), "{}", f.family);
            assert!(ab_glyph::FontRef::try_from_slice(f.bytes).is_ok(), "{} {} parses", f.family, f.style);
        }
        assert!(CRAFT_FONTS.iter().any(|f| f.has_script("Jpan")), "craft-fonts carries Japanese faces");
    }
}

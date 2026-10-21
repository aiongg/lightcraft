//! Design tokens (colours, sizes, fonts). Values measured from black-box observation of the
//! reference app's dark theme (see plan/lightroom/10-observed-ui.md); everything is our own code.
//!
//! Themes come in two families, picked by the Appearance Mode setting (Settings ▸ Interface):
//! - dark: **Charcoal** (the default, the observed dark look) and **Midnight** (near black);
//! - light: **Silver** (light grey chrome) and **Paper** (near white).

use std::sync::OnceLock;

use craft_ui::direction;
use craft_ui::fonts::{CraftFace, CraftFace as EmbeddedFace, Emoji, Family, I18nScripts, Stack, SystemFonts, UiFonts};

use egui::{Color32, CornerRadius, FontData, FontFamily, FontId, Stroke, Visuals};

pub const FONT_SEMIBOLD: &str = "semibold";

/// Shared colour-label palette for badges, thumbnail surrounds and feedback.
pub fn label_color(label: lightcraft_catalog::ColorLabel) -> Color32 {
    use lightcraft_catalog::ColorLabel;
    match label {
        ColorLabel::Red => Color32::from_rgb(222, 72, 72),
        ColorLabel::Yellow => Color32::from_rgb(232, 196, 58),
        ColorLabel::Green => Color32::from_rgb(88, 176, 92),
        ColorLabel::Blue => Color32::from_rgb(72, 130, 222),
        ColorLabel::Purple => Color32::from_rgb(158, 100, 210),
    }
}

/// A translucent label colour over thumbnail chrome; selection remains brighter.
pub fn label_background(base: Color32, label: Option<lightcraft_catalog::ColorLabel>, selected: bool) -> Color32 {
    label.map_or(base, |l| base.lerp_to_gamma(label_color(l), if selected { 0.32 } else { 0.22 }))
}

/// Label confirmations use a pale colour and dark text, distinct from neutral/error HUDs.
pub fn label_toast_colors(label: lightcraft_catalog::ColorLabel) -> (Color32, Color32) {
    (Color32::WHITE.lerp_to_gamma(label_color(label), 0.3), Color32::from_rgb(36, 24, 24))
}

#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    /// Top bar, side panels, bottom bar, tool strip.
    pub chrome: Color32,
    /// Photo canvas (detail view) and filmstrip.
    pub canvas: Color32,
    /// Grid background behind the cells.
    pub grid_bg: Color32,
    pub cell: Color32,
    pub cell_selected: Color32,
    pub divider: Color32,
    pub inset: Color32,
    pub field: Color32,
    pub field_border: Color32,
    pub button: Color32,
    pub button_border: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub tool_active: Color32,
    pub text: Color32,
    pub text_label: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    /// Icon glyphs in the chrome.
    pub icon: Color32,
    pub track: Color32,
    pub thumb: Color32,
    pub thumb_hover: Color32,
    pub accent: Color32,
    pub star: Color32,
    pub pick: Color32,
    pub reject: Color32,
    /// Cautionary notices (e.g. a raw shown from its embedded preview): a muted amber.
    pub caution: Color32,
    pub mask_overlay: Color32,
    // metrics (points)
    pub top_bar_h: f32,
    pub bottom_bar_h: f32,
    pub panel_w: f32,
    pub strip_w: f32,
    pub slider_row_h: f32,
    pub section_h: f32,
    pub film_h: f32,
}

/// An interface theme. Each belongs to the dark or the light family.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeKind {
    #[default]
    Charcoal,
    Midnight,
    Silver,
    Paper,
}

impl ThemeKind {
    pub const ALL: [ThemeKind; 4] = [ThemeKind::Charcoal, ThemeKind::Midnight, ThemeKind::Silver, ThemeKind::Paper];

    pub fn is_dark(self) -> bool {
        matches!(self, ThemeKind::Charcoal | ThemeKind::Midnight)
    }

    /// The theme's id (settings value and command suffix).
    pub fn id(self) -> &'static str {
        match self {
            ThemeKind::Charcoal => "charcoal",
            ThemeKind::Midnight => "midnight",
            ThemeKind::Silver => "silver",
            ThemeKind::Paper => "paper",
        }
    }

    /// The English display name.
    pub fn label(self) -> &'static str {
        match self {
            ThemeKind::Charcoal => "Charcoal",
            ThemeKind::Midnight => "Midnight",
            ThemeKind::Silver => "Silver",
            ThemeKind::Paper => "Paper",
        }
    }
}

impl Default for Tokens {
    fn default() -> Self {
        Tokens {
            chrome: Color32::from_rgb(0x2d, 0x2d, 0x2d),
            canvas: Color32::from_rgb(0x1c, 0x1c, 0x1c),
            grid_bg: Color32::from_rgb(0x0f, 0x0f, 0x0f),
            cell: Color32::from_rgb(0x1c, 0x1c, 0x1c),
            cell_selected: Color32::from_rgb(0x2d, 0x2d, 0x2d),
            divider: Color32::from_rgb(0x1c, 0x1c, 0x1c),
            inset: Color32::from_rgb(0x23, 0x23, 0x23),
            field: Color32::from_rgb(0x22, 0x22, 0x22),
            field_border: Color32::from_rgb(0x3c, 0x3c, 0x3c),
            button: Color32::from_rgb(0x24, 0x24, 0x24),
            button_border: Color32::from_rgb(0x4c, 0x4c, 0x4c),
            hover: Color32::from_rgb(0x3a, 0x3a, 0x3a),
            pressed: Color32::from_rgb(0x3f, 0x3f, 0x3f),
            tool_active: Color32::from_rgb(0x3f, 0x3f, 0x3f),
            text: Color32::from_rgb(0xe2, 0xe2, 0xe2),
            text_label: Color32::from_rgb(0xbc, 0xbc, 0xbc),
            text_dim: Color32::from_rgb(0x8e, 0x8e, 0x8e),
            text_disabled: Color32::from_rgb(0x5c, 0x5c, 0x5c),
            icon: Color32::from_rgb(0x9a, 0x9a, 0x9a),
            track: Color32::from_rgb(0x5a, 0x5a, 0x5a),
            thumb: Color32::from_rgb(0xa0, 0xa0, 0xa0),
            thumb_hover: Color32::from_rgb(0xe0, 0xe0, 0xe0),
            accent: Color32::from_rgb(0x01, 0x65, 0xdd),
            star: Color32::from_rgb(0xd8, 0xd8, 0xd8),
            pick: Color32::from_rgb(0xf0, 0xf0, 0xf0),
            reject: Color32::from_rgb(0xe0, 0x4a, 0x4a),
            caution: Color32::from_rgb(0xe3, 0xa8, 0x3c),
            mask_overlay: Color32::from_rgba_unmultiplied(0xe0, 0x20, 0x30, 110),
            top_bar_h: 42.0,
            bottom_bar_h: 48.0,
            panel_w: 270.0,
            strip_w: 48.0,
            slider_row_h: 45.0,
            section_h: 57.0,
            film_h: 142.0,
        }
    }
}

impl Tokens {
    /// The tokens of `kind`. Metrics are shared; only colours change.
    pub fn for_kind(kind: ThemeKind) -> Tokens {
        let rgb = |v: u32| Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8);
        let base = Tokens::default();
        match kind {
            ThemeKind::Charcoal => base,
            ThemeKind::Midnight => Tokens {
                chrome: rgb(0x1e1e1e),
                canvas: rgb(0x121212),
                grid_bg: rgb(0x080808),
                cell: rgb(0x121212),
                cell_selected: rgb(0x262626),
                divider: rgb(0x0e0e0e),
                inset: rgb(0x181818),
                field: rgb(0x161616),
                field_border: rgb(0x333333),
                button: rgb(0x191919),
                button_border: rgb(0x404040),
                hover: rgb(0x2e2e2e),
                pressed: rgb(0x343434),
                tool_active: rgb(0x343434),
                ..base
            },
            ThemeKind::Silver => Tokens {
                chrome: rgb(0xe6e6e6),
                canvas: rgb(0xcfcfcf),
                grid_bg: rgb(0xd9d9d9),
                cell: rgb(0xcbcbcb),
                cell_selected: rgb(0xb4b4b4),
                divider: rgb(0xc4c4c4),
                inset: rgb(0xdcdcdc),
                field: rgb(0xf6f6f6),
                field_border: rgb(0xb8b8b8),
                button: rgb(0xf2f2f2),
                button_border: rgb(0xb0b0b0),
                hover: rgb(0xd2d2d2),
                pressed: rgb(0xc4c4c4),
                tool_active: rgb(0xcacaca),
                text: rgb(0x1c1c1c),
                text_label: rgb(0x383838),
                text_dim: rgb(0x6a6a6a),
                text_disabled: rgb(0xa2a2a2),
                icon: rgb(0x555555),
                track: rgb(0xa8a8a8),
                thumb: rgb(0x5e5e5e),
                thumb_hover: rgb(0x262626),
                accent: rgb(0x0a62d0),
                star: rgb(0x3a3a3a),
                pick: rgb(0x2a2a2a),
                reject: rgb(0xc83434),
                caution: rgb(0xb0761a),
                ..base
            },
            ThemeKind::Paper => Tokens {
                chrome: rgb(0xfafafa),
                canvas: rgb(0xececec),
                grid_bg: rgb(0xf2f2f2),
                cell: rgb(0xe8e8e8),
                cell_selected: rgb(0xd2d2d2),
                divider: rgb(0xe0e0e0),
                inset: rgb(0xf0f0f0),
                field: rgb(0xffffff),
                field_border: rgb(0xcccccc),
                button: rgb(0xffffff),
                button_border: rgb(0xc6c6c6),
                hover: rgb(0xebebeb),
                pressed: rgb(0xdedede),
                tool_active: rgb(0xe2e2e2),
                text: rgb(0x161616),
                text_label: rgb(0x343434),
                text_dim: rgb(0x707070),
                text_disabled: rgb(0xababab),
                icon: rgb(0x585858),
                track: rgb(0xbdbdbd),
                thumb: rgb(0x606060),
                thumb_hover: rgb(0x202020),
                accent: rgb(0x0a62d0),
                star: rgb(0x3a3a3a),
                pick: rgb(0x2a2a2a),
                reject: rgb(0xc83434),
                caution: rgb(0xb0761a),
                ..base
            },
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_default()
    }
    pub fn font(&self, size: f32) -> FontId {
        FontId::proportional(size)
    }
    pub fn semibold(&self, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(FONT_SEMIBOLD.into()))
    }
}

/// craft-fonts' interface faces (see `build.rs`): the CJK interface faces cut to the characters
/// of the catalogs, the faces of other scripts, symbols and emoji; empty unless built with
/// `CRAFT_FONTS_DIR`.
static UI_FACES: &[EmbeddedFace] = include!(concat!(env!("OUT_DIR"), "/ui_faces.rs"));

/// ISO 15924 `Zsye`, the emoji faces' script tag.
const EMOJI: &str = "Zsye";

fn has_script(face: &CraftFace, script: &str) -> bool {
    face.scripts.iter().any(|s| *s == script)
}

/// Whether the face is for Chinese, Japanese or Korean (`Hant-HK` included).
fn is_cjk(face: &CraftFace) -> bool {
    face.scripts.iter().any(|s| matches!(s.split('-').next(), Some("Hans" | "Hant" | "Hani" | "Jpan" | "Kore")))
}

/// The craft-fonts faces of the interface, emoji aside. A craft-fonts checkout without interface
/// faces gives no CJK cuts; the interface then takes the document fonts' CJK faces.
pub(crate) fn craft_faces() -> &'static [CraftFace] {
    static FACES: OnceLock<Vec<CraftFace>> = OnceLock::new();
    FACES.get_or_init(|| {
        let cuts = UI_FACES.iter().any(is_cjk);
        let documents = lightcraft_engine::CRAFT_FONTS.iter().filter(|face| !cuts && face.is_cjk()).copied().map(CraftFace::from);
        UI_FACES.iter().filter(|face| !has_script(face, EMOJI)).copied().chain(documents).collect()
    })
}

/// The interface table's emoji faces, asked after the installed fonts: Noto Color Emoji, then the
/// monochrome Noto Emoji.
fn emoji() -> Emoji {
    let face = |colour: bool| {
        UI_FACES
            .iter()
            .find(|face| has_script(face, EMOJI) && face.family.contains("Color") == colour)
            .map(|face| FontData { index: face.index, ..FontData::from_static(face.bytes) })
    };
    Emoji { color: face(true), monochrome: face(false), ..Emoji::default() }
}

/// LightCraft's UI fonts for `language`: Inter, then the craft-fonts faces and installed fonts in
/// the suite's order ([`craft_ui::fonts`]). Han characters take the forms of the interface
/// language's region.
pub(crate) fn font_stack(language: crate::i18n::Locale, system: SystemFonts) -> Stack {
    ui_fonts(language, craft_faces(), emoji()).system_fonts(system).stack()
}

/// [`font_stack`]'s configuration with the given craft-fonts faces.
pub(crate) fn ui_fonts(language: crate::i18n::Locale, craft: &[CraftFace], emoji: Emoji) -> UiFonts<'static> {
    let codes: Vec<&str> = crate::i18n::Locale::ALL.iter().map(|l| l.code()).collect();
    UiFonts::new(&I18nScripts)
        .app_face("Inter", FontData::from_static(include_bytes!("../../../assets/fonts/Inter-Regular.ttf")), &[FontFamily::Proportional])
        .app_face(
            "Inter-SemiBold",
            FontData::from_static(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf")),
            &[FontFamily::Name(FONT_SEMIBOLD.into())],
        )
        .family(Family::named(FONT_SEMIBOLD).weight(600))
        .craft_faces(craft)
        .emoji(emoji)
        .languages(&codes)
        .ui_language(language.code())
}

/// Install the UI fonts and the interface direction for the current interface language
/// (`CRAFT_UI_DIRECTION=rtl` mirrors a left-to-right language, for checking layouts). Call it
/// again when the language changes.
pub fn install_fonts(ctx: &egui::Context) {
    let language = crate::i18n::language();
    font_stack(language, SystemFonts::AfterCraft).install(ctx);
    direction::apply(ctx, direction::resolve(language.code(), direction::Setting::Language));
}

/// The font families used by this app instance.
pub fn font_credits() -> String {
    let mut families = vec!["Inter"];
    for family in UI_FACES.iter().map(|f| f.family).chain(lightcraft_engine::CRAFT_FONTS.iter().map(|f| f.family)) {
        if !families.contains(&family) {
            families.push(family);
        }
    }
    families.join(" / ")
}

/// Points one mouse-wheel notch (a line) scrolls.
pub const WHEEL_LINE_POINTS: f32 = 100.0;

/// Share of its visible size one mouse-wheel notch scrolls the photo grid and the filmstrip (Lightroom
/// Classic: three notches move a photo from one edge to the other, at any window size).
pub const WHEEL_STRIP_SHARE: f32 = 1.0 / 3.0;

/// Zoom per wheel point (`exp(speed × points)`): egui's 1/200 at its stock 40-point notch, rescaled so
/// one notch still zooms the same amount now that it scrolls [`WHEEL_LINE_POINTS`].
pub const WHEEL_ZOOM_PER_POINT: f32 = 40.0 / 200.0 / WHEEL_LINE_POINTS;

fn theme_id() -> egui::Id {
    egui::Id::unique("lightcraft-theme")
}

/// The theme `ctx` is styled with (none before the first [`apply_kind`]).
pub fn current(ctx: &egui::Context) -> Option<ThemeKind> {
    ctx.data(|d| d.get_temp::<ThemeKind>(theme_id()))
}

/// Style `ctx` with the default theme.
pub fn apply(ctx: &egui::Context) {
    apply_kind(ctx, ThemeKind::default());
}

/// Style `ctx` with `kind`. The visuals go to both of egui's styles, so egui following the
/// system's dark/light setting on its own never swaps in its stock look.
pub fn apply_kind(ctx: &egui::Context, kind: ThemeKind) {
    // Keep photo zoom shortcuts from resizing egui itself.
    ctx.options_mut(|o| o.zoom_with_keyboard = false);
    // A mouse wheel notch scrolls 100 points (egui's default 40 took 50+ notches through an open
    // Edit panel; Lightroom moves about this much per notch). Trackpads report points and are
    // unaffected.
    // A modifier-wheel (and the loupe's plain-wheel) zoom notch stays about 1.22x, as it was at 40 points.
    ctx.options_mut(|o| {
        o.input_options.line_scroll_speed = WHEEL_LINE_POINTS;
        o.input_options.scroll_zoom_speed = WHEEL_ZOOM_PER_POINT;
    });
    let t = Tokens::for_kind(kind);
    ctx.data_mut(|d| {
        d.insert_temp(egui::Id::NULL, t);
        d.insert_temp(theme_id(), kind);
    });
    let dark = kind.is_dark();
    let mut v = if dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.chrome;
    v.window_fill = t.chrome;
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.inset;
    v.window_stroke = Stroke::new(1.0, t.button_border);
    v.window_corner_radius = CornerRadius::same(6);
    v.menu_corner_radius = CornerRadius::same(6);
    v.selection.bg_fill = t.accent.gamma_multiply(0.6);
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.override_text_color = Some(t.text_label);
    let (popup, window) = if dark { (140, 160) } else { (50, 60) };
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: Color32::from_black_alpha(popup) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 30, spread: 0, color: Color32::from_black_alpha(window) };
    let w = &mut v.widgets;
    craft_ui::theme::WidgetPalette { noninteractive: t.chrome, inactive: t.button, hovered: t.hover, active: t.pressed, open: t.hover }.apply_to(
        w,
        CornerRadius::same(4),
        Stroke::new(1.0, t.text_label),
    );
    w.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    w.inactive.bg_stroke = Stroke::new(1.0, t.button_border);
    ctx.set_visuals_of(egui::Theme::Dark, v.clone());
    ctx.set_visuals_of(egui::Theme::Light, v);
    ctx.set_theme(if dark { egui::Theme::Dark } else { egui::Theme::Light });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(8.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Button, FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Small, FontId::proportional(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, FontId::new(16.0, FontFamily::Name(FONT_SEMIBOLD.into())));
        s.animation_time = 0.08;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Locale;

    /// Every theme scrolls 100 points a wheel notch, and a zoom notch stays egui's stock ~1.22x
    /// (`exp(40 / 200)`) instead of growing with the longer scroll.
    #[test]
    fn a_wheel_notch_scrolls_further_but_zooms_as_before_in_every_theme() {
        for kind in [ThemeKind::Charcoal, ThemeKind::Midnight, ThemeKind::Silver, ThemeKind::Paper] {
            let ctx = egui::Context::default();
            apply_kind(&ctx, kind);
            let (line, zoom) = ctx.options(|o| (o.input_options.line_scroll_speed, o.input_options.scroll_zoom_speed));
            assert_eq!(line, WHEEL_LINE_POINTS, "{kind:?}");
            assert!(((zoom * line).exp() - (40.0_f32 / 200.0).exp()).abs() < 1e-4, "{kind:?}: a notch zooms {}", (zoom * line).exp());
        }
    }

    /// The UI families, installed in a fresh context for `language` without installed fonts, after
    /// one frame so the faces are loaded.
    fn fonts_ctx(language: Locale) -> egui::Context {
        let ctx = egui::Context::default();
        font_stack(language, SystemFonts::Off).install(&ctx);
        let mut frame = ctx.run_ui(egui::RawInput::default(), |_| {});
        frame.textures_delta.clear();
        ctx
    }

    fn semibold_family() -> FontFamily {
        FontFamily::Name(FONT_SEMIBOLD.into())
    }

    /// #736: the release input must carry Thai, not silently skip the check with an older pin.
    /// Exercise the Thai characters of the reported folder name through every egui family.
    #[test]
    fn thai_folder_name_has_glyphs_in_every_ui_family() {
        if !craft_faces().iter().any(|face| has_script(face, "Thai")) {
            lightcraft_testkit::skip!(Fonts, "built without CRAFT_FONTS_DIR: no craft-fonts Thai face to check");
            return;
        }
        let ctx = fonts_ctx(Locale::En);
        ctx.fonts_mut(|fonts| {
            for family in [FontFamily::Proportional, semibold_family(), FontFamily::Monospace] {
                let id = FontId::new(13.0, family);
                for ch in "Studio34 ถ่ายภาพงานแต่งงาน".chars().filter(|ch| !ch.is_ascii()) {
                    assert!(fonts.has_glyph(&id, ch), "{id:?} has {ch}");
                }
            }
        });
    }

    /// The width of `text` in `family` at the body size, and the ink its glyphs put in the font
    /// atlas (summed coverage).
    fn width_and_ink(ctx: &egui::Context, text: &str, family: FontFamily) -> (f32, u64) {
        ctx.fonts_mut(|fonts| {
            let galley = fonts.layout_no_wrap(text.into(), FontId::new(13.0, family), Color32::WHITE);
            let atlas = fonts.image();
            let mut ink = 0_u64;
            for glyph in galley.rows.iter().flat_map(|row| row.glyphs.iter()) {
                let (min, max) = (glyph.uv_rect.min, glyph.uv_rect.max);
                for y in min[1]..max[1] {
                    for x in min[0]..max[0] {
                        ink += u64::from(atlas[(usize::from(x), usize::from(y))].a());
                    }
                }
            }
            (galley.size().x, ink)
        })
    }

    /// Strong text in a script whose fallback face is a variable font is really heavier: an Arabic
    /// tab name used to be drawn at regular weight beside a bold Latin one.
    #[test]
    fn strong_arabic_text_is_heavier_than_regular() {
        if !craft_faces().iter().any(|face| has_script(face, "Arab")) {
            lightcraft_testkit::skip!(Fonts, "built without a craft-fonts face for Arabic");
            return;
        }
        let ctx = fonts_ctx(Locale::En);
        let word = "الصور";
        let (regular_width, regular_ink) = width_and_ink(&ctx, word, FontFamily::Proportional);
        let (strong_width, strong_ink) = width_and_ink(&ctx, word, semibold_family());
        assert!(strong_width > regular_width, "strong Arabic is wider than regular: strong {strong_width} px, regular {regular_width} px");
        assert!(strong_ink as f64 > regular_ink as f64 * 1.05, "strong Arabic has more ink than regular: strong {strong_ink}, regular {regular_ink}");
        // Latin keeps Inter in both weights.
        let (regular, _) = width_and_ink(&ctx, "Library", FontFamily::Proportional);
        let (strong, _) = width_and_ink(&ctx, "Library", semibold_family());
        assert!(strong > regular, "strong Latin stays wider than regular: {strong} px, {regular} px");
    }
}

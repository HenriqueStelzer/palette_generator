#![allow(dead_code)]

use three_d::Srgba;
use wasm_bindgen::JsCast;
use web_sys::window;

#[derive(Clone, Debug)]
pub struct ColorOklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}
impl ColorOklch {
    pub fn to_css(&self) -> String {
        format!("oklch({:.2} {} {})", self.l, self.c, self.h)
    }
    pub fn to_srgba(&self, a: u8) -> Srgba {
        let mut pixel = [self.l, self.c, self.h];
        colcon::convert_space(colcon::Space::OKLCH, colcon::Space::SRGB, &mut pixel);
        Srgba::new(
            (pixel[0] * 255.0) as u8,
            (pixel[1] * 255.0) as u8,
            (pixel[2] * 255.0) as u8,
            a,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Tonality {
    pub muted: ColorOklch,
    pub base: ColorOklch,
    pub prominent: ColorOklch,
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub surface: Tonality,
    pub text: Tonality,
    pub neutral: Tonality,
    pub brand: Tonality,
    pub semantic: Semantic,
}
impl Palette {
    pub fn sync_theme_css(&self) {
        // Default usage: theme.active_palette().sync_theme_css();
        if let Some(window) = window() {
            let document = window.document().unwrap();
            let root = document.document_element().unwrap();
            let html_element = root.dyn_into::<web_sys::HtmlElement>().unwrap();
            let style = html_element.style();

            style
                .set_property("--surface-alt", &self.surface.muted.to_css())
                .unwrap();
            style
                .set_property("--surface-bg", &self.surface.base.to_css())
                .unwrap();
            style
                .set_property("--surface-main", &self.surface.prominent.to_css())
                .unwrap();
            style
                .set_property("--text-muted", &self.text.muted.to_css())
                .unwrap();
            style
                .set_property("--text", &self.text.base.to_css())
                .unwrap();
            style
                .set_property("--text-contrast", &self.text.prominent.to_css())
                .unwrap();
            style
                .set_property("--shadow", &self.neutral.muted.to_css())
                .unwrap();
            style
                .set_property("--neutral", &self.neutral.base.to_css())
                .unwrap();
            style
                .set_property("--light", &self.neutral.prominent.to_css())
                .unwrap();
            style
                .set_property("--accent-1", &self.brand.muted.to_css())
                .unwrap();
            style
                .set_property("--brand", &self.brand.base.to_css())
                .unwrap();
            style
                .set_property("--accent-2", &self.brand.prominent.to_css())
                .unwrap();
            style
                .set_property("--error", &self.semantic.error.to_css())
                .unwrap();
            style
                .set_property("--warning", &self.semantic.warning.to_css())
                .unwrap();
            style
                .set_property("--success", &self.semantic.success.to_css())
                .unwrap();
            style
                .set_property("--info", &self.semantic.info.to_css())
                .unwrap();
        }
    }
}

#[derive(Clone, Debug)]
pub struct Semantic {
    pub error: ColorOklch,
    pub warning: ColorOklch,
    pub success: ColorOklch,
    pub info: ColorOklch,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub light: Palette,
    pub dark: Palette,
    pub mode: Mode,
}
impl Theme {
    pub fn active_palette(&self) -> &Palette {
        match self.mode {
            Mode::Light => &self.light,
            Mode::Dark => &self.dark,
        }
    }
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        };
    }
    pub fn generate(hue: f32, contrast: f32, brightness: f32, variety: f32) -> Self {
        // ── Shared derived values ────────────────────────────────────────────

        // Surfaces get a whisper of the hue so they feel "in family" without
        // being coloured. variety scales it, but the ceiling (0.012) keeps it
        // imperceptible as an actual colour — just a warmth/coolness.
        let surf_c = variety * 0.012;

        // Brand chroma: the main colour punch. At variety=0 it's 0.12 (still
        // distinctly coloured in OKLCH), at variety=1 it reaches 0.27 (vivid).
        let brand_c = 0.12 + variety * 0.15;

        // Semantic colours (error/warning/success/info) scale mildly with
        // variety so they stay readable in both flat and vivid themes.
        let sem_c = 0.17 + variety * 0.06;

        // ── Light palette ────────────────────────────────────────────────────

        // Surface lightness: near-white. brightness lifts the floor.
        // Range: base ∈ [0.93, 0.98]; muted is 0.04 below; prominent hugs 1.0.
        let ls_base = 0.93 + brightness * 0.05;
        let ls_muted = ls_base - 0.04;
        let ls_prom = (ls_base + 0.03).min(1.0);

        // Text lightness in light mode: dark tones.
        // contrast pulls the darkest text deeper (lower L = higher contrast).
        // muted ∈ [0.05 (max contrast), 0.25 (min contrast)]
        let lt_muted = 0.05 + (1.0 - contrast) * 0.20;
        let lt_base = lt_muted + 0.15; // medium-dark body text
        let lt_prom = lt_muted + 0.38; // de-emphasised / placeholder text

        // Brand in light: mid-range L so it reads on bright surfaces.
        // brightness lifts it slightly to match the surface mood.
        let lb_base = 0.50 + brightness * 0.08;
        let lb_muted = lb_base - 0.10; // darker accent
        let lb_prom = lb_base + 0.18; // lighter tint

        // Neutrals in light: shadow anchors to near-black, highlight near-white.
        // contrast deepens the shadow (lower L = harder shadow).
        let ln_shadow = 0.05 + (1.0 - contrast) * 0.15;
        let ln_mid = 0.77 + brightness * 0.06;
        let ln_light = (0.96 + brightness * 0.03).min(1.0);

        // ── Dark palette ────────────────────────────────────────────────────

        // Surface lightness in dark: near-black floor.
        // brightness raises the floor so high-brightness dark themes feel less cave-like.
        // prominent is the deepest (darkest) layer; muted is the shallowest.
        let ds_prom = 0.08 + brightness * 0.05; // deepest surface ∈ [0.08, 0.13]
        let ds_base = ds_prom + 0.04;
        let ds_muted = ds_prom + 0.08;

        // Text in dark mode: near-white tones.
        // contrast lifts the brightest text higher (more pop against dark bg).
        // muted is the *brightest* text tier here (matching the original convention).
        let dt_muted = 0.92 + contrast * 0.06; // ∈ [0.92, 0.98]
        let dt_base = dt_muted - 0.10;
        let dt_prom = dt_muted - 0.32; // de-emphasised text, lower L on dark = greyer

        // Brand in dark: lifted lightness so it doesn't drown in the dark surface.
        // Chroma is dialled down slightly — high chroma on dark can feel harsh.
        let db_base = 0.63 + brightness * 0.06;
        let db_muted = db_base - 0.10;
        let db_prom = db_base + 0.14;
        let dark_brand_c = brand_c * 0.82; // subtle desaturation on dark bg

        // Neutrals in dark: very deep shadow, mid-tone surfaces.
        let dn_shadow = 0.02 + (1.0 - contrast) * 0.05;
        let dn_mid = 0.27 + brightness * 0.05;
        let dn_light = 0.20 + brightness * 0.04;

        // ── Assemble ─────────────────────────────────────────────────────────

        Theme {
            mode: Mode::Light,
            light: Palette {
                surface: Tonality {
                    muted: ColorOklch {
                        l: ls_muted,
                        c: surf_c,
                        h: hue,
                    },
                    base: ColorOklch {
                        l: ls_base,
                        c: surf_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: ls_prom,
                        c: 0.0,
                        h: 0.0,
                    },
                },
                text: Tonality {
                    muted: ColorOklch {
                        l: lt_prom,
                        c: surf_c * 0.5,
                        h: hue,
                    },
                    base: ColorOklch {
                        l: lt_base,
                        c: surf_c * 0.5,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: lt_muted,
                        c: surf_c * 0.5,
                        h: hue,
                    },
                },
                neutral: Tonality {
                    muted: ColorOklch {
                        l: ln_shadow,
                        c: 0.0,
                        h: 0.0,
                    },
                    base: ColorOklch {
                        l: ln_mid,
                        c: surf_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: ln_light,
                        c: 0.0,
                        h: 0.0,
                    },
                },
                brand: Tonality {
                    muted: ColorOklch {
                        l: lb_muted,
                        c: brand_c,
                        h: hue,
                    },
                    base: ColorOklch {
                        l: lb_base,
                        c: brand_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: lb_prom,
                        c: brand_c * 0.65,
                        h: hue,
                    },
                },
                semantic: Semantic {
                    error: ColorOklch {
                        l: 0.55,
                        c: sem_c,
                        h: 25.0,
                    },
                    warning: ColorOklch {
                        l: 0.72,
                        c: sem_c * 0.85,
                        h: 85.0,
                    },
                    success: ColorOklch {
                        l: 0.55,
                        c: sem_c,
                        h: 145.0,
                    },
                    info: ColorOklch {
                        l: 0.55,
                        c: sem_c,
                        h: 220.0,
                    },
                },
            },
            dark: Palette {
                surface: Tonality {
                    muted: ColorOklch {
                        l: ds_muted,
                        c: surf_c,
                        h: hue,
                    },
                    base: ColorOklch {
                        l: ds_base,
                        c: surf_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: ds_prom,
                        c: surf_c,
                        h: hue,
                    },
                },
                text: Tonality {
                    muted: ColorOklch {
                        l: dt_muted,
                        c: 0.0,
                        h: 0.0,
                    },
                    base: ColorOklch {
                        l: dt_base,
                        c: surf_c * 0.4,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: dt_prom,
                        c: surf_c * 0.6,
                        h: hue,
                    },
                },
                neutral: Tonality {
                    muted: ColorOklch {
                        l: dn_shadow,
                        c: 0.0,
                        h: 0.0,
                    },
                    base: ColorOklch {
                        l: dn_mid,
                        c: surf_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: dn_light,
                        c: surf_c,
                        h: hue,
                    },
                },
                brand: Tonality {
                    muted: ColorOklch {
                        l: db_muted,
                        c: dark_brand_c,
                        h: hue,
                    },
                    base: ColorOklch {
                        l: db_base,
                        c: dark_brand_c,
                        h: hue,
                    },
                    prominent: ColorOklch {
                        l: db_prom,
                        c: dark_brand_c * 0.72,
                        h: hue,
                    },
                },
                semantic: Semantic {
                    error: ColorOklch {
                        l: 0.68,
                        c: sem_c,
                        h: 25.0,
                    },
                    warning: ColorOklch {
                        l: 0.80,
                        c: sem_c * 0.85,
                        h: 85.0,
                    },
                    success: ColorOklch {
                        l: 0.68,
                        c: sem_c,
                        h: 145.0,
                    },
                    info: ColorOklch {
                        l: 0.68,
                        c: sem_c,
                        h: 220.0,
                    },
                },
            },
        }
    }
}

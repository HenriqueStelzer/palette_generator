#![allow(dead_code)]

use web_sys::window;
use wasm_bindgen::JsCast;
use three_d::Srgba;

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

            style.set_property("--surface-alt", &self.surface.muted.to_css()).unwrap();
            style.set_property("--surface-bg", &self.surface.base.to_css()).unwrap();
            style.set_property("--surface-main", &self.surface.prominent.to_css()).unwrap();
            style.set_property("--text-muted", &self.text.muted.to_css()).unwrap();
            style.set_property("--text", &self.text.base.to_css()).unwrap();
            style.set_property("--text-contrast", &self.text.prominent.to_css()).unwrap();
            style.set_property("--shadow", &self.neutral.muted.to_css()).unwrap();
            style.set_property("--neutral", &self.neutral.base.to_css()).unwrap();
            style.set_property("--light", &self.neutral.prominent.to_css()).unwrap();
            style.set_property("--accent-1", &self.brand.muted.to_css()).unwrap();
            style.set_property("--brand", &self.brand.base.to_css()).unwrap();
            style.set_property("--accent-2", &self.brand.prominent.to_css()).unwrap();
            style.set_property("--error", &self.semantic.error.to_css()).unwrap();
            style.set_property("--warning", &self.semantic.warning.to_css()).unwrap();
            style.set_property("--success", &self.semantic.success.to_css()).unwrap();
            style.set_property("--info", &self.semantic.info.to_css()).unwrap();
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
        pub fn new() -> Self {
        Theme {
            mode: Mode::Light,
            light: Palette {
                surface: Tonality {
                    muted:    ColorOklch { l: 0.93, c: 0.01, h: 240.0 },
                    base: ColorOklch { l: 0.97, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 1.00, c: 0.00, h:   0.0 },
                },
                text: Tonality {
                    muted:    ColorOklch { l: 0.15, c: 0.01, h: 240.0 },
                    base: ColorOklch { l: 0.30, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 0.55, c: 0.01, h: 240.0 },
                },
                neutral: Tonality {
                    muted:    ColorOklch { l: 0.20, c: 0.00, h:   0.0 },
                    base: ColorOklch { l: 0.80, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 0.99, c: 0.00, h:   0.0 },
                },
                brand: Tonality {
                    muted:    ColorOklch { l: 0.45, c: 0.18, h: 250.0 },
                    base: ColorOklch { l: 0.55, c: 0.22, h: 250.0 },
                    prominent:   ColorOklch { l: 0.75, c: 0.15, h: 250.0 },
                },
                semantic: Semantic {
                    error:   ColorOklch { l: 0.55, c: 0.22, h:  25.0 },
                    warning: ColorOklch { l: 0.75, c: 0.17, h:  85.0 },
                    success: ColorOklch { l: 0.55, c: 0.18, h: 145.0 },
                    info:    ColorOklch { l: 0.55, c: 0.18, h: 220.0 },
                },
            },
            dark: Palette {
                surface: Tonality {
                    muted:    ColorOklch { l: 0.18, c: 0.01, h: 240.0 },
                    base: ColorOklch { l: 0.13, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 0.10, c: 0.01, h: 240.0 },
                },
                text: Tonality {
                    muted:    ColorOklch { l: 0.98, c: 0.00, h:   0.0 },
                    base: ColorOklch { l: 0.88, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 0.65, c: 0.01, h: 240.0 },
                },
                neutral: Tonality {
                    muted:    ColorOklch { l: 0.05, c: 0.00, h:   0.0 },
                    base: ColorOklch { l: 0.30, c: 0.01, h: 240.0 },
                    prominent:   ColorOklch { l: 0.22, c: 0.01, h: 240.0 },
                },
                brand: Tonality {
                    muted:    ColorOklch { l: 0.55, c: 0.18, h: 250.0 },
                    base: ColorOklch { l: 0.68, c: 0.18, h: 250.0 },
                    prominent:   ColorOklch { l: 0.80, c: 0.14, h: 250.0 },
                },
                semantic: Semantic {
                    error:   ColorOklch { l: 0.68, c: 0.18, h:  25.0 },
                    warning: ColorOklch { l: 0.82, c: 0.14, h:  85.0 },
                    success: ColorOklch { l: 0.68, c: 0.15, h: 145.0 },
                    info:    ColorOklch { l: 0.68, c: 0.15, h: 220.0 },
                },
            },
        }
    }
    pub fn active_palette(&self) -> &Palette {
        match self.mode {
            Mode::Light => &self.light,
            Mode::Dark  => &self.dark,
        }
    }
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        };
    }
}


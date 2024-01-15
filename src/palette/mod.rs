use ratatui::style::{Color, Style};

/// This module contains the color palettes that are supported by [base16](). These can be
/// converted into styles used by Ratatui.

// TODO: Only a few palettes are implemented.
//  - Except for Grubbox, everything past the "a"s is not done

pub mod default;

#[cfg(feature = "apprentice")]
pub mod apprentice;
#[cfg(feature = "atelier")]
pub mod atelier;
#[cfg(feature = "atlas")]
pub mod atlas;
#[cfg(feature = "black_metal")]
pub mod black_metal;
#[cfg(feature = "brogrammer")]
pub mod brogrammer;
#[cfg(feature = "brush_trees")]
pub mod brush_trees;
#[cfg(feature = "circus")]
pub mod circus;
#[cfg(feature = "classic")]
pub mod classic;
#[cfg(feature = "codeschool")]
pub mod codeschool;
#[cfg(feature = "colors")]
pub mod colors;
#[cfg(feature = "cupertino")]
pub mod cupertino;
#[cfg(feature = "danqing")]
pub mod danqing;
#[cfg(feature = "darcula")]
pub mod darcula;
#[cfg(feature = "darkviolet")]
pub mod darkviolet;
#[cfg(feature = "dracula")]
pub mod dracula;
#[cfg(feature = "equilibrium")]
pub mod equilibrium;
#[cfg(feature = "espresso")]
pub mod espresso;
#[cfg(feature = "eva")]
pub mod eva;
#[cfg(feature = "framer")]
pub mod framer;
#[cfg(feature = "fruit_soda")]
pub mod fruit_soda;
#[cfg(feature = "gigavolt")]
pub mod gigavolt;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gruvbox")]
pub mod gruvbox;
#[cfg(feature = "hardcore")]
pub mod hardcore;
#[cfg(feature = "heetch")]
pub mod heetch;
#[cfg(feature = "helios")]
pub mod helios;
#[cfg(feature = "horizon")]
pub mod horizon;
#[cfg(feature = "humanoid")]
pub mod humanoid;
#[cfg(feature = "ia")]
pub mod ia;
#[cfg(feature = "icy")]
pub mod icy;
#[cfg(feature = "kimber")]
pub mod kimber;
#[cfg(feature = "materia")]
pub mod materia;
#[cfg(feature = "material_theme")]
pub mod material_theme;
#[cfg(feature = "material_vivid")]
pub mod material_vivid;
#[cfg(feature = "mellow")]
pub mod mellow;
#[cfg(feature = "mexico_light")]
pub mod mexico_light;
#[cfg(feature = "nebula")]
pub mod nebula;
#[cfg(feature = "nord")]
pub mod nord;
#[cfg(feature = "nova")]
pub mod nova;
#[cfg(feature = "one_light")]
pub mod one_light;
#[cfg(feature = "onedark")]
pub mod onedark;
#[cfg(feature = "outrun")]
pub mod outrun;
#[cfg(feature = "papercolor")]
pub mod papercolor;
#[cfg(feature = "pasque")]
pub mod pasque;
#[cfg(feature = "pinky")]
pub mod pinky;
#[cfg(feature = "porple")]
pub mod porple;
#[cfg(feature = "purpledream")]
pub mod purpledream;
#[cfg(feature = "qualia")]
pub mod qualia;
#[cfg(feature = "rebecca")]
pub mod rebecca;
#[cfg(feature = "rose_pine")]
pub mod rose_pine;
#[cfg(feature = "sagelight")]
pub mod sagelight;
#[cfg(feature = "sakura")]
pub mod sakura;
#[cfg(feature = "sandcastle")]
pub mod sandcastle;
#[cfg(feature = "shades_of_purple")]
pub mod shades_of_purple;
#[cfg(feature = "silk")]
pub mod silk;
#[cfg(feature = "snazzy")]
pub mod snazzy;
#[cfg(feature = "solarflare")]
pub mod solarflare;
#[cfg(feature = "solarized")]
pub mod solarized;
#[cfg(feature = "summercamp")]
pub mod summercamp;
#[cfg(feature = "summerfruit")]
pub mod summerfruit;
#[cfg(feature = "synth_midnight")]
pub mod synth_midnight;
#[cfg(feature = "tender")]
pub mod tender;
#[cfg(feature = "twilight")]
pub mod twilight;
#[cfg(feature = "unikitty")]
pub mod unikitty;
#[cfg(feature = "vice")]
pub mod vice;
#[cfg(feature = "windows")]
pub mod windows;
#[cfg(feature = "woodland")]
pub mod woodland;
#[cfg(feature = "xcode_dust")]
pub mod xcode_dust;
#[cfg(feature = "zenburn")]
pub mod zenburn;

/// The universal representation of a Base16 color palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Color {
    Shade(Base16Shade),
    Accent(Base16Accent),
}

/// Every Base16 color palette contains 8 "shades". These are split between 4 "dark" and 4 "light"
/// shades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Shade {
    Dark(Shade),
    Light(Shade),
}

/// Base16 shades are split into two 4-value gradients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shade {
    Darkest,
    Darker,
    Lighter,
    Lightest,
}

/// Every Base16 color palette contains 8 "accents".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Accent {
    Accent00,
    Accent01,
    Accent02,
    Accent03,
    Accent04,
    Accent05,
    Accent06,
    Accent07,
}

impl Base16Color {
    pub const fn index(self) -> u8 {
        match self {
            Base16Color::Shade(Base16Shade::Dark(Shade::Darkest)) => 0,
            Base16Color::Shade(Base16Shade::Dark(Shade::Darker)) => 1,
            Base16Color::Shade(Base16Shade::Dark(Shade::Lighter)) => 2,
            Base16Color::Shade(Base16Shade::Dark(Shade::Lightest)) => 3,
            Base16Color::Shade(Base16Shade::Light(Shade::Darkest)) => 4,
            Base16Color::Shade(Base16Shade::Light(Shade::Darker)) => 5,
            Base16Color::Shade(Base16Shade::Light(Shade::Lighter)) => 6,
            Base16Color::Shade(Base16Shade::Light(Shade::Lightest)) => 7,
            Base16Color::Accent(Base16Accent::Accent00) => 8,
            Base16Color::Accent(Base16Accent::Accent01) => 9,
            Base16Color::Accent(Base16Accent::Accent02) => 10,
            Base16Color::Accent(Base16Accent::Accent03) => 11,
            Base16Color::Accent(Base16Accent::Accent04) => 12,
            Base16Color::Accent(Base16Accent::Accent05) => 13,
            Base16Color::Accent(Base16Accent::Accent06) => 14,
            Base16Color::Accent(Base16Accent::Accent07) => 15,
        }
    }

    pub const fn from_index(i: u8) -> Self {
        match i {
            0 => Base16Color::Shade(Base16Shade::Dark(Shade::Darkest)),
            1 => Base16Color::Shade(Base16Shade::Dark(Shade::Darker)),
            2 => Base16Color::Shade(Base16Shade::Dark(Shade::Lighter)),
            3 => Base16Color::Shade(Base16Shade::Dark(Shade::Lightest)),
            4 => Base16Color::Shade(Base16Shade::Light(Shade::Darkest)),
            5 => Base16Color::Shade(Base16Shade::Light(Shade::Darker)),
            6 => Base16Color::Shade(Base16Shade::Light(Shade::Lighter)),
            7 => Base16Color::Shade(Base16Shade::Light(Shade::Lightest)),
            8 => Base16Color::Accent(Base16Accent::Accent00),
            9 => Base16Color::Accent(Base16Accent::Accent01),
            10 => Base16Color::Accent(Base16Accent::Accent02),
            11 => Base16Color::Accent(Base16Accent::Accent03),
            12 => Base16Color::Accent(Base16Accent::Accent04),
            13 => Base16Color::Accent(Base16Accent::Accent05),
            14 => Base16Color::Accent(Base16Accent::Accent06),
            15 => Base16Color::Accent(Base16Accent::Accent07),
            // Maybe use a default value?
            _ => panic!("Unknown color code!"),
        }
    }
}

pub(crate) use create::create_palette;

mod create {
    macro_rules! create_palette {
        ($name:ident,
        $s01:literal,
        $s02:literal,
        $s03:literal,
        $s04:literal,
        $s05:literal,
        $s06:literal,
        $s07:literal,
        $s08:literal,
        $s09:literal,
        $s10:literal,
        $s11:literal,
        $s12:literal,
        $s13:literal,
        $s14:literal,
        $s15:literal,
        $s16:literal,
        ) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name(pub crate::palette::Base16Color);

            impl From<crate::palette::Base16Color> for $name {
                fn from(other: crate::palette::Base16Color) -> Self {
                    Self(other)
                }
            }

            impl From<$name> for crate::palette::Base16Color {
                fn from(other: $name) -> Self {
                    other.0
                }
            }

            impl crate::palette::Palette for $name {
                fn to_rgb(self) -> (u8, u8, u8) {
                    match self.into() {
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Darkest)) => hex_literal::hex!($s01).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Darker)) => hex_literal::hex!($s02).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Lighter)) => hex_literal::hex!($s03).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Lightest)) => hex_literal::hex!($s04).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Darkest)) => hex_literal::hex!($s05).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Darker)) => hex_literal::hex!($s06).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Lighter)) => hex_literal::hex!($s07).into(),
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Lightest)) => hex_literal::hex!($s08).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent00) => hex_literal::hex!($s09).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent01) => hex_literal::hex!($s10).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent02) => hex_literal::hex!($s11).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent03) => hex_literal::hex!($s12).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent04) => hex_literal::hex!($s13).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent05) => hex_literal::hex!($s14).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent06) => hex_literal::hex!($s15).into(),
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent07) => hex_literal::hex!($s16).into(),
                    }
                }

                fn to_rgb_str(self) -> &'static str{
                    match self.into() {
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Darkest)) => $s01,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Darker)) => $s02,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Lighter)) => $s03,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Dark(crate::palette::Shade::Lightest)) => $s04,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Darkest)) => $s05,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Darker)) => $s06,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Lighter)) => $s07,
                        crate::palette::Base16Color::Shade(crate::palette::Base16Shade::Light(crate::palette::Shade::Lightest)) => $s08,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent00) => $s09,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent01) => $s10,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent02) => $s11,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent03) => $s12,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent04) => $s13,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent05) => $s14,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent06) => $s15,
                        crate::palette::Base16Color::Accent(crate::palette::Base16Accent::Accent07) => $s16,
                    }
                }
            }
        };
    }

    pub(crate) use create_palette;
}


/// A simple trait to extend the iterface of GruvboxColor.
pub trait Palette: From<Base16Color> + Into<Base16Color> + Copy {
    fn to_rgb(self) -> (u8, u8, u8);
    fn to_rgb_str(self) -> &'static str;

    // Provided methods
    /// Creates a style from this color and the given color. This color will be used as the
    /// foreground while the given color will be used as the background.
    fn full_style(self, other: Self) -> Style {
        Style::new().fg(self.to_rgb_color()).bg(other.to_rgb_color())
    }

    /// Creates a style using the default foreground and background colors.
    fn default_style() -> Style {
        Self::default_fg().full_style(Self::default_bg())
    }

    /// Creates a style using this color as the foreground and the default background color as the
    /// background.
    fn fg_style(self) -> Style {
        self.full_style(Self::default_bg())
    }

    /// Creates a style using this color as the background and the default foreground color as the
    /// foreground.
    fn bg_style(self) -> Style {
        Self::default_fg().full_style(self)
    }

    /// Creates an indexed color.
    fn to_rgb_color(self) -> Color {
        let (r, g, b) = self.to_rgb();
        Color::Rgb(r, g, b)
    }

    /// Creates an indexed color.
    fn to_indexed_color(self) -> Color {
        Color::Indexed(self.into().index())
    }

    fn default_fg() -> Self {
        Self::light_3()
    }

    fn default_bg() -> Self {
        Self::dark_2()
    }

    fn dark_1() -> Self {
        Base16Color::Shade(Base16Shade::Dark(Shade::Darkest)).into()
    }

    fn dark_2() -> Self {
        Base16Color::Shade(Base16Shade::Dark(Shade::Darker)).into()
    }

    fn dark_3() -> Self {
        Base16Color::Shade(Base16Shade::Dark(Shade::Lighter)).into()
    }

    fn dark_4() -> Self {
        Base16Color::Shade(Base16Shade::Dark(Shade::Lightest)).into()
    }

    fn light_1() -> Self {
        Base16Color::Shade(Base16Shade::Light(Shade::Darkest)).into()
    }

    fn light_2() -> Self {
        Base16Color::Shade(Base16Shade::Light(Shade::Darker)).into()
    }

    fn light_3() -> Self {
        Base16Color::Shade(Base16Shade::Light(Shade::Lighter)).into()
    }

    fn light_4() -> Self {
        Base16Color::Shade(Base16Shade::Light(Shade::Lightest)).into()
    }
}

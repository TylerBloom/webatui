use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasPalette {
    Atlas(Atlas),
}

create_palette! {
    Atlas,
    "002635",
    "00384d",
    "517F8D",
    "6C8B91",
    "869696",
    "a1a19a",
    "e6e6dc",
    "fafaf8",
    "ff5a67",
    "f08e48",
    "ffcc1b",
    "7fc06e",
    "5dd7b9",
    "14747e",
    "9a70a4",
    "c43060",
}

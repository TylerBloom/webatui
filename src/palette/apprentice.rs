use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprenticePalette {
    Apprentice(Apprentice),
}

create_palette! {
    Apprentice,
    "262626",
    "AF5F5F",
    "5F875F",
    "87875F",
    "5F87AF",
    "5F5F87",
    "5F8787",
    "6C6C6C",
    "444444",
    "FF8700",
    "87AF87",
    "FFFFAF",
    "87AFD7",
    "8787AF",
    "5FAFAF",
    "BCBCBC",
}

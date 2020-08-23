use super::common::{Scalar, Color};
use super::font::Font;

#[derive(Debug, Copy, Clone)]
pub enum Overflow {
    Hidden,
    Ellipsis,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Ellipsis
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Wrap {
    None,
    Word,
    Letter,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::Word
    }
}

#[derive(Debug, Default, Clone)]
pub struct Text {
    justify: bool,
    overflow: Overflow,
    wrap_mode: Wrap,
    letter_spacing: Scalar,
    line_spacing: Scalar,
    paragraph_spacing: Scalar,
    tab_width: Scalar,
    indent_width: Scalar,
    whitespace_width: Scalar,
    color: Color,
    underline: Option<Color>,
    strikethrough: Option<Color>,
    font: Font,
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    justify: Option<bool>,
    overflow: Option<Overflow>,
    wrap_mode: Option<Wrap>,
    letter_spacing: Option<Scalar>,
    line_spacing: Option<Scalar>,
    paragraph_spacing: Option<Scalar>,
    tab_width: Option<Scalar>,
    indent_width: Option<Scalar>,
    whitespace_width: Option<Scalar>,
    color: Option<Color>,
    underline: Option<Option<Color>>,
    strikethrough: Option<Option<Color>>,
    font: Option<Font>,
}

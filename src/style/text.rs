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
}

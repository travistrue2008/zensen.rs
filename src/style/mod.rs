pub mod common;
pub mod border;
pub mod layout;
pub mod font;
pub mod text;

use self::common::*;
use self::border::Border;
use self::font::Font;
use self::layout::Layout;
use self::text::Text;

#[derive(Debug, Copy, Clone)]
pub enum SelectMode {
    All,
    Text,
}

impl Default for SelectMode {
    fn default() -> Self {
        SelectMode::All
    }
}

#[derive(Debug, Clone)]
pub enum Cursor {
    Default,
    None,
    Menu,
    Help,
    Pointer,
    Progress,
    Hourglass,
    Text,
    Copy,
    Move,
    Grab,
    Grabbing,
    Prohibited,
    Enlarge,
    ResizeHorizontal,
    ResizeVertical,
    ZoomIn,
    ZoomOut,
    Url(String),
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Default
    }
}

#[derive(Debug, Clone)]
pub enum Position {
    Flow,
    Absolute(Sides<Scalar>),
}

impl Default for Position {
    fn default() -> Self {
        Position::Flow
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Span {
    Amount(u32),
    End,
}

impl Default for Span {
    fn default() -> Self {
        Span::Amount(1)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Style {
    capture: bool, // capture absolute-positioned child nodes
    detect_cursor: bool,
    select_mode: SelectMode,
    cursor: Cursor,
    position: Position,
    layer: i8,
    opacity: f32,
    margin: Sides<Scalar>,
    padding: Sides<Scalar>,
    border: Border,
    span_rows: Span,
    span_columns: Span,
    layout: Layout,
    font: Font,
    text: Text,
}

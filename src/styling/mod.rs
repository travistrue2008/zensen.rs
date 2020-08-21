mod common;
mod border;
mod layout;
mod frame;
mod font;
mod text;

use self::frame::Frame;
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

#[derive(Debug, Default, Clone)]
pub struct Style {
    detect_cursor: bool,
    select_mode: SelectMode,
    cursor: Cursor,
    layer: i8,
    opacity: f32,
    frame: Frame,
    layout: Layout,
    text: Text,
}

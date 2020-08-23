#[derive(Debug, Clone)]
pub enum Family {
    Default,
    Url(String),
}

impl Default for Family {
    fn default() -> Self {
        Family::Default
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Stretch {
    Normal,
    Expanded,
    Condensed,
}

impl Default for Stretch {
    fn default() -> Self {
        Stretch::Normal
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Modifier {
    Normal,
    Italic,
    Oblique,
    Both,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::Normal
    }
}

#[derive(Debug, Default, Clone)]
pub struct Font {
    family: Family,
    stretch: Stretch,
    style: Modifier,
    weight: u16,
}

#[derive(Debug, Default, Clone)]
pub struct Modifiers {
    family: Option<Family>,
    stretch: Option<Stretch>,
    style: Option<Modifier>,
    weight: Option<u16>,
}

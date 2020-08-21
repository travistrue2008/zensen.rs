use super::common::Color;

#[derive(Debug, Default, Copy, Clone)]
pub struct GradientSegment {
    factor: f32,
    color: Color,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Fill {
    None,
    Color(Color),
    Image,
    LinearGradient(f32, Vec<GradientSegment>),
    RadialGradient(Vec<GradientSegment>),
}

impl Default for Fill {
    fn default() -> Self {
        Fill::None
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Background {
    fill: Fill,
}

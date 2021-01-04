use super::common::{Scalar, Sides, Color};

#[derive(Debug, Default, Copy, Clone)]
pub struct CornerMeasures {
    upper_left: Scalar,
    upper_right: Scalar,
    lower_right: Scalar,
    lower_left: Scalar,
}

impl CornerMeasures {
    pub fn new(size: Scalar) -> CornerMeasures {
        CornerMeasures {
            upper_left: size,
            upper_right: size,
            lower_right: size,
            lower_left: size,
        }
    }

    pub fn make(upper_left: Scalar, upper_right: Scalar, lower_right: Scalar, lower_left: Scalar) -> CornerMeasures {
        CornerMeasures {
            upper_left,
            upper_right,
            lower_right,
            lower_left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Solid,
    Dotted { size: u32, spacing: u32 },
    Dashed { size: u32, spacing: u32 },
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Solid
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Border {
    kind: Sides<Kind>,
    color: Sides<Color>,
    width: Sides<Scalar>,
    radius: CornerMeasures,
}

impl Border {
    pub fn new(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: Sides::new(width),
            color: Sides::new(color),
            kind: Sides::new(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn top(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: Sides::top(width),
            color: Sides::top(color),
            kind: Sides::top(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn right(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: Sides::right(width),
            color: Sides::right(color),
            kind: Sides::right(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn bottom(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: Sides::bottom(width),
            color: Sides::bottom(color),
            kind: Sides::bottom(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn left(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: Sides::left(width),
            color: Sides::left(color),
            kind: Sides::left(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }
}

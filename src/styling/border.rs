use super::common::{Scalar, SideMeasures, Color};

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
    width: SideMeasures<Scalar>,
    color: SideMeasures<Color>,
    kind: SideMeasures<Kind>,
    radius: CornerMeasures,
}

impl Border {
    pub fn new(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: SideMeasures::new(width),
            color: SideMeasures::new(color),
            kind: SideMeasures::new(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn top(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: SideMeasures::top(width),
            color: SideMeasures::top(color),
            kind: SideMeasures::top(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn right(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: SideMeasures::right(width),
            color: SideMeasures::right(color),
            kind: SideMeasures::right(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn bottom(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: SideMeasures::bottom(width),
            color: SideMeasures::bottom(color),
            kind: SideMeasures::bottom(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }

    pub fn left(width: Scalar, color: Color, kind: Kind) -> Border {
        Border {
            width: SideMeasures::left(width),
            color: SideMeasures::left(color),
            kind: SideMeasures::left(kind),
            radius: CornerMeasures::new(Default::default()),
        }
    }
}

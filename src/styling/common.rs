#[derive(Debug, Copy, Clone)]
pub enum Align {
    Stretch,
    Start,
    End,
    Center,
}

impl Default for Align {
    fn default() -> Self {
        Align::Stretch
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Scalar {
    Auto,
    Zero,
    Pc(i32),
    Px(i32),
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar::Auto
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            red, green, blue, alpha,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct SideMeasures<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T: Default + Copy> SideMeasures<T> {
    pub fn new(size: T) -> SideMeasures<T> {
        SideMeasures {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }

    pub fn make(top: T, right: T, bottom: T, left: T) -> SideMeasures<T> {
        SideMeasures {
            top, right, bottom, left,
        }
    }

    pub fn top(size: T) -> SideMeasures<T> {
        SideMeasures {
            top: size,
            right: Default::default(),
            bottom: Default::default(),
            left: Default::default(),
        }
    }

    pub fn right(size: T) -> SideMeasures<T> {
        SideMeasures {
            top: Default::default(),
            right: size,
            bottom: Default::default(),
            left: Default::default(),
        }
    }

    pub fn bottom(size: T) -> SideMeasures<T> {
        SideMeasures {
            top: Default::default(),
            right: Default::default(),
            bottom: size,
            left: Default::default(),
        }
    }

    pub fn left(size: T) -> SideMeasures<T> {
        SideMeasures {
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default(),
            left: size,
        }
    }

    pub fn make_hor_vert(top_bottom: T, left_right: T) -> SideMeasures<T> {
        SideMeasures {
            top: top_bottom,
            right: left_right,
            bottom: top_bottom,
            left: left_right,
        }
    }
}

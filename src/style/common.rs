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
pub struct Sides<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T: Default + Copy> Sides<T> {
    pub fn new(size: T) -> Sides<T> {
        Sides {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }

    pub fn make(top: T, right: T, bottom: T, left: T) -> Sides<T> {
        Sides {
            top, right, bottom, left,
        }
    }

    pub fn make_xy(x: T, y: T) -> Sides<T> {
        Sides {
            top: x,
            right: y,
            bottom: y,
            left: x,
        }
    }

    pub fn top(size: T) -> Sides<T> {
        Sides {
            top: size,
            right: Default::default(),
            bottom: Default::default(),
            left: Default::default(),
        }
    }

    pub fn right(size: T) -> Sides<T> {
        Sides {
            top: Default::default(),
            right: size,
            bottom: Default::default(),
            left: Default::default(),
        }
    }

    pub fn bottom(size: T) -> Sides<T> {
        Sides {
            top: Default::default(),
            right: Default::default(),
            bottom: size,
            left: Default::default(),
        }
    }

    pub fn left(size: T) -> Sides<T> {
        Sides {
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default(),
            left: size,
        }
    }
}

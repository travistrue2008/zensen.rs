use super::common::{Align, Scalar};

#[derive(Debug, Copy, Clone)]
pub enum Flow {
    Row,
    Column,
}

impl Default for Flow {
    fn default() -> Self {
        Flow::Row
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Order {
    Forward,
    Reverse,
}

impl Default for Order {
    fn default() -> Self {
        Order::Forward
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Gap {
    horizontal: Scalar,
    vertical: Scalar,
}

impl Gap {
    pub fn uniform(amount: Scalar) -> Gap {
        Gap {
            horizontal: amount,
            vertical: amount,
        }
    }

    pub fn horizontal(amount: Scalar) -> Gap {
        Gap {
            horizontal: amount,
            vertical: Default::default(),
        }
    }

    pub fn vertical(amount: Scalar) -> Gap {
        Gap {
            horizontal: Default::default(),
            vertical: amount,
        }
    }

    pub fn both(horizontal: Scalar, vertical: Scalar) -> Gap {
        Gap { horizontal, vertical }
    }
}

/* TODO: min(), max(), minmax() */
#[derive(Debug, Copy, Clone)]
pub enum SizePolicy {
    Auto,
    Zero,
    Fr(u32),
    Pc(u32),
    Px(u32),
}

impl Default for SizePolicy {
    fn default() -> Self {
        SizePolicy::Auto
    }
}

#[derive(Debug, Clone)]
pub enum Template {
    Horizontal(Vec<SizePolicy>),
    Vertical(Vec<SizePolicy>),
}

impl Default for Template {
    fn default() -> Self {
        Template::Horizontal(vec![SizePolicy::Fr(1)])
    }
}

#[derive(Debug, Default, Clone)]
pub struct Layout {
    flow: Flow,
    order: Order,
    gap: Gap,
    template: Template,
    align_horizontal: Align,
    align_vertical: Align,
}

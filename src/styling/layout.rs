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

#[derive(Debug, Copy, Clone)]
pub enum SizePolicy {
    Auto,
    Zero,
    Fr(u32),
    Pc(u32),
    Px(u32),
    Min(Scalar),
    Max(Scalar),
    Range(Scalar, Scalar),
}

impl SizePolicy {
    pub fn repeat(count: usize, policy: SizePolicy) -> Vec<SizePolicy> {
        [policy].repeat(count)
    }
}

impl Default for SizePolicy {
    fn default() -> Self {
        SizePolicy::Auto
    }
}

#[derive(Debug, Clone)]
pub enum Template {
    Columns(Vec<SizePolicy>),
    Rows(Vec<SizePolicy>),
}

impl Default for Template {
    fn default() -> Self {
        Template::Columns(vec![SizePolicy::Fr(1)])
    }
}

#[derive(Debug, Default, Clone)]
pub struct Layout {
    flow: Flow,
    order: Order,
    gap: Gap,
    template: Template,
    align_h: Align,
    align_v: Align,
}

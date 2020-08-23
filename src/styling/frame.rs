use super::border::Border;
use super::common::{Align, Scalar, SideMeasures};

#[derive(Debug, Clone)]
pub enum Position {
    Flow,
    Absolute(SideMeasures<Scalar>),
}

impl Default for Position {
    fn default() -> Self {
        Position::Flow
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
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
pub struct Frame {
    capture: bool,
    overflow: Overflow,
    position: Position,
    margin: SideMeasures<Scalar>,
    padding: SideMeasures<Scalar>,
    border: Border,
    span_rows: Span,
    span_columns: Span,
    align_self_h: Option<Align>,
    align_self_v: Option<Align>,
}

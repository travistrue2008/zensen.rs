use std::ops::Add;

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
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Focus {
    None,
    Enabled,
    Index(u8),
}

impl Default for Focus {
    fn default() -> Self {
        Focus::None
    }
}

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

#[derive(Debug, Copy, Clone)]
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
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Default
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Flow,
    Absolute(Scalar, Scalar, Scalar, Scalar),
}

impl Default for Position {
    fn default() -> Self {
        Position::Flow
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

#[derive(Debug, Copy, Clone)]
pub enum BorderKind {
    Solid,
    Dotted { size: u32, spacing: u32 },
    Dashed { size: u32, spacing: u32 },
}

impl Default for BorderKind {
    fn default() -> Self {
        BorderKind::Solid
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct GradientSegment {
    factor: f32,
    color: Color,
}

#[derive(Debug, Clone)]
pub enum BackgroundFill {
    None,
    Color(Color),
    Image,
    LinearGradient(f32, Vec<GradientSegment>),
    RadialGradient(Vec<GradientSegment>),
}

impl Default for BackgroundFill {
    fn default() -> Self {
        BackgroundFill::None
    }
}

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
pub enum LayoutOverflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for LayoutOverflow {
    fn default() -> Self {
        LayoutOverflow::Visible
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LayoutFlow {
    Row,
    Column,
}

impl Default for LayoutFlow {
    fn default() -> Self {
        LayoutFlow::Row
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LayoutOrder {
    Forward,
    Reverse,
}

impl Default for LayoutOrder {
    fn default() -> Self {
        LayoutOrder::Forward
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SizePolicy {
    Auto,
    Zero,
    MinContent,
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
pub enum FontFamily {
    Default,
    Url(String),
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::Default
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FontStretch {
    Normal,
    Expanded,
    Condensed,
}

impl Default for FontStretch {
    fn default() -> Self {
        FontStretch::Normal
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FontModifier {
    Normal,
    Italic,
    Oblique,
    Both,
}

impl Default for FontModifier {
    fn default() -> Self {
        FontModifier::Normal
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextOverflow {
    Hidden,
    Ellipsis,
}

impl Default for TextOverflow {
    fn default() -> Self {
        TextOverflow::Ellipsis
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextWrap {
    None,
    Word,
    Letter,
}

impl Default for TextWrap {
    fn default() -> Self {
        TextWrap::Word
    }
}

#[derive(Debug, Default, Clone)]
pub struct Style {
    detect_cursor: bool,
    capture_absolute: bool,
    select_mode: SelectMode,
    focus: Focus,
    cursor: Cursor,
    position: Position,
    layer: u8,
    opacity: f32,

    spacing_top: Scalar,
    spacing_right: Scalar,
    spacing_bottom: Scalar,
    spacing_left: Scalar,

    border_kind_top: BorderKind,
    border_kind_right: BorderKind,
    border_kind_bottom: BorderKind,
    border_kind_left: BorderKind,
    border_color_top: Color,
    border_color_right: Color,
    border_color_bottom: Color,
    border_color_left: Color,
    border_width_top: Scalar,
    border_width_right: Scalar,
    border_width_bottom: Scalar,
    border_width_left: Scalar,
    border_radius_upper_left: Scalar,
    border_radius_upper_right: Scalar,
    border_radius_lower_right: Scalar,
    border_radius_lower_left: Scalar,

    background_fill: BackgroundFill,

    span_rows: Span,
    span_columns: Span,
    align_self_h: Align,
    align_self_v: Align,

    layout_overflow_x: LayoutOverflow,
    layout_overflow_y: LayoutOverflow,
    layout_flow: LayoutFlow,
    layout_order: LayoutOrder,
    layout_gap_h: Scalar,
    layout_gap_v: Scalar,
    layout_align_h: Align,
    layout_align_v: Align,
    layout_rows: Vec<SizePolicy>,
    layout_columns: Vec<SizePolicy>,

    font_family: FontFamily,
    font_stretch: FontStretch,
    font_style: FontModifier,
    font_weight: u16,

    justify: bool,
    letter_spacing: Scalar,
    line_spacing: Scalar,
    paragraph_spacing: Scalar,
    whitespace_width: Scalar,
    tab_width: Scalar,
    text_overflow: TextOverflow,
    text_wrap_mode: TextWrap,
    text_color: Color,
    underline: Option<Color>,
    strikethrough: Option<Color>,
}

#[derive(Debug, Default, Clone)]
pub struct StyleBuilder {
    detect_cursor: Option<bool>,
    capture_absolute: Option<bool>,
    select_mode: Option<SelectMode>,
    focus: Option<Focus>,
    cursor: Option<Cursor>,
    position: Option<Position>,
    layer: Option<u8>,
    opacity: Option<f32>,

    spacing_top: Option<Scalar>,
    spacing_right: Option<Scalar>,
    spacing_bottom: Option<Scalar>,
    spacing_left: Option<Scalar>,

    border_kind_top: Option<BorderKind>,
    border_kind_right: Option<BorderKind>,
    border_kind_bottom: Option<BorderKind>,
    border_kind_left: Option<BorderKind>,
    border_color_top: Option<Color>,
    border_color_right: Option<Color>,
    border_color_bottom: Option<Color>,
    border_color_left: Option<Color>,
    border_width_top: Option<Scalar>,
    border_width_right: Option<Scalar>,
    border_width_bottom: Option<Scalar>,
    border_width_left: Option<Scalar>,
    border_radius_upper_left: Option<Scalar>,
    border_radius_upper_right: Option<Scalar>,
    border_radius_lower_right: Option<Scalar>,
    border_radius_lower_left: Option<Scalar>,

    background_fill: Option<BackgroundFill>,

    span_rows: Option<Span>,
    span_columns: Option<Span>,
    align_self_h: Option<Align>,
    align_self_v: Option<Align>,

    layout_overflow_x: Option<LayoutOverflow>,
    layout_overflow_y: Option<LayoutOverflow>,
    layout_flow: Option<LayoutFlow>,
    layout_order: Option<LayoutOrder>,
    layout_gap_h: Option<Scalar>,
    layout_gap_v: Option<Scalar>,
    layout_align_h: Option<Align>,
    layout_align_v: Option<Align>,
    layout_rows: Option<Vec<SizePolicy>>,
    layout_columns: Option<Vec<SizePolicy>>,

    font_family: Option<FontFamily>,
    font_stretch: Option<FontStretch>,
    font_style: Option<FontModifier>,
    font_weight: Option<u16>,

    justify: Option<bool>,
    letter_spacing: Option<Scalar>,
    line_spacing: Option<Scalar>,
    paragraph_spacing: Option<Scalar>,
    whitespace_width: Option<Scalar>,
    tab_width: Option<Scalar>,
    text_overflow: Option<TextOverflow>,
    text_wrap_mode: Option<TextWrap>,
    text_color: Option<Color>,
    underline: Option<Option<Color>>,
    strikethrough: Option<Option<Color>>,
}

impl StyleBuilder {
    pub fn detect_cursor(mut self, v: bool) -> StyleBuilder {
        self.detect_cursor = Some(v);
        self
    }

    pub fn capture_absolute(mut self, v: bool) -> StyleBuilder {
        self.capture_absolute = Some(v);
        self
    }

    pub fn select_mode(mut self, v: SelectMode) -> StyleBuilder {
        self.select_mode = Some(v);
        self
    }

    pub fn focus(mut self, v: Focus) -> StyleBuilder {
        self.focus = Some(v);
        self
    }

    pub fn cursor(mut self, v: Cursor) -> StyleBuilder {
        self.cursor = Some(v);
        self
    }

    pub fn position(mut self, v: Position) -> StyleBuilder {
        self.position = Some(v);
        self
    }

    pub fn layer(mut self, v: u8) -> StyleBuilder {
        self.layer = Some(v);
        self
    }

    pub fn opacity(mut self, v: f32) -> StyleBuilder {
        self.opacity = Some(v);
        self
    }

    /* spacing */

    pub fn spacing_top(mut self, v: Scalar) -> StyleBuilder {
        self.spacing_top = Some(v);
        self
    }

    pub fn spacing_right(mut self, v: Scalar) -> StyleBuilder {
        self.spacing_right = Some(v);
        self
    }

    pub fn spacing_bottom(mut self, v: Scalar) -> StyleBuilder {
        self.spacing_bottom = Some(v);
        self
    }

    pub fn spacing_left(mut self, v: Scalar) -> StyleBuilder {
        self.spacing_left = Some(v);
        self
    }

    pub fn spacing_hv(mut self, h: Scalar, v: Scalar) -> StyleBuilder {
        self.spacing_top = Some(v);
        self.spacing_right = Some(h);
        self.spacing_bottom = Some(h);
        self.spacing_left = Some(v);
        self
    }

    pub fn spacing(mut self, v: Scalar) -> StyleBuilder {
        self.spacing_top = Some(v);
        self.spacing_right = Some(v);
        self.spacing_bottom = Some(v);
        self.spacing_left = Some(v);
        self
    }

    /* border_kind */

    pub fn border_kind_top(mut self, v: BorderKind) -> StyleBuilder {
        self.border_kind_top = Some(v);
        self
    }

    pub fn border_kind_right(mut self, v: BorderKind) -> StyleBuilder {
        self.border_kind_right = Some(v);
        self
    }

    pub fn border_kind_bottom(mut self, v: BorderKind) -> StyleBuilder {
        self.border_kind_bottom = Some(v);
        self
    }

    pub fn border_kind_left(mut self, v: BorderKind) -> StyleBuilder {
        self.border_kind_left = Some(v);
        self
    }

    pub fn border_kind_hv(mut self, h: BorderKind, v: BorderKind) -> StyleBuilder {
        self.border_kind_top = Some(v);
        self.border_kind_right = Some(h);
        self.border_kind_bottom = Some(h);
        self.border_kind_left = Some(v);
        self
    }

    pub fn border_kind(mut self, v: BorderKind) -> StyleBuilder {
        self.border_kind_top = Some(v);
        self.border_kind_right = Some(v);
        self.border_kind_bottom = Some(v);
        self.border_kind_left = Some(v);
        self
    }

    /* border_color */

    pub fn border_color_top(mut self, v: Color) -> StyleBuilder {
        self.border_color_top = Some(v);
        self
    }

    pub fn border_color_right(mut self, v: Color) -> StyleBuilder {
        self.border_color_right = Some(v);
        self
    }

    pub fn border_color_bottom(mut self, v: Color) -> StyleBuilder {
        self.border_color_bottom = Some(v);
        self
    }

    pub fn border_color_left(mut self, v: Color) -> StyleBuilder {
        self.border_color_left = Some(v);
        self
    }

    pub fn border_color_hv(mut self, h: Color, v: Color) -> StyleBuilder {
        self.border_color_top = Some(v);
        self.border_color_right = Some(h);
        self.border_color_bottom = Some(h);
        self.border_color_left = Some(v);
        self
    }

    pub fn border_color(mut self, v: Color) -> StyleBuilder {
        self.border_color_top = Some(v);
        self.border_color_right = Some(v);
        self.border_color_bottom = Some(v);
        self.border_color_left = Some(v);
        self
    }

    /* border_width */

    pub fn border_width_top(mut self, v: Scalar) -> StyleBuilder {
        self.border_width_top = Some(v);
        self
    }

    pub fn border_width_right(mut self, v: Scalar) -> StyleBuilder {
        self.border_width_right = Some(v);
        self
    }

    pub fn border_width_bottom(mut self, v: Scalar) -> StyleBuilder {
        self.border_width_bottom = Some(v);
        self
    }

    pub fn border_width_left(mut self, v: Scalar) -> StyleBuilder {
        self.border_width_left = Some(v);
        self
    }

    pub fn border_width_hv(mut self, h: Scalar, v: Scalar) -> StyleBuilder {
        self.border_width_top = Some(v);
        self.border_width_right = Some(h);
        self.border_width_bottom = Some(h);
        self.border_width_left = Some(v);
        self
    }

    pub fn border_width(mut self, v: Scalar) -> StyleBuilder {
        self.border_width_top = Some(v);
        self.border_width_right = Some(v);
        self.border_width_bottom = Some(v);
        self.border_width_left = Some(v);
        self
    }

    /* border_radius */

    pub fn border_radius_upper_left(mut self, v: Scalar) -> StyleBuilder {
        self.border_radius_upper_left = Some(v);
        self
    }

    pub fn border_radius_upper_right(mut self, v: Scalar) -> StyleBuilder {
        self.border_radius_upper_right = Some(v);
        self
    }

    pub fn border_radius_lower_right(mut self, v: Scalar) -> StyleBuilder {
        self.border_radius_lower_right = Some(v);
        self
    }

    pub fn border_radius_lower_left(mut self, v: Scalar) -> StyleBuilder {
        self.border_radius_lower_left = Some(v);
        self
    }

    pub fn border_radius(mut self, v: Scalar) -> StyleBuilder {
        self.border_radius_upper_left = Some(v);
        self.border_radius_upper_right = Some(v);
        self.border_radius_lower_right = Some(v);
        self.border_radius_lower_left = Some(v);
        self
    }

    /* border */

    pub fn border_top(mut self, width: Scalar, kind: BorderKind, color: Color) -> StyleBuilder {
        self.border_width_top = Some(width);
        self.border_kind_top = Some(kind);
        self.border_color_top = Some(color);
        self
    }

    pub fn border_right(mut self, width: Scalar, kind: BorderKind, color: Color) -> StyleBuilder {
        self.border_width_right = Some(width);
        self.border_kind_right = Some(kind);
        self.border_color_right = Some(color);
        self
    }

    pub fn border_bottom(mut self, width: Scalar, kind: BorderKind, color: Color) -> StyleBuilder {
        self.border_width_bottom = Some(width);
        self.border_kind_bottom = Some(kind);
        self.border_color_bottom = Some(color);
        self
    }

    pub fn border_left(mut self, width: Scalar, kind: BorderKind, color: Color) -> StyleBuilder {
        self.border_width_left = Some(width);
        self.border_kind_left = Some(kind);
        self.border_color_left = Some(color);
        self
    }

    pub fn border(mut self, width: Scalar, kind: BorderKind, color: Color) -> StyleBuilder {
        self.border_width_top = Some(width);
        self.border_width_right = Some(width);
        self.border_width_bottom = Some(width);
        self.border_width_left = Some(width);

        self.border_kind_top = Some(kind);
        self.border_kind_right = Some(kind);
        self.border_kind_bottom = Some(kind);
        self.border_kind_left = Some(kind);

        self.border_color_top = Some(color);
        self.border_color_right = Some(color);
        self.border_color_bottom = Some(color);
        self.border_color_left = Some(color);

        self
    }

    /* background */

    pub fn background_fill(mut self, v: BackgroundFill) -> StyleBuilder {
        self.background_fill = Some(v);
        self
    }

    /* spans */

    pub fn span_rows(mut self, v: Span) -> StyleBuilder {
        self.span_rows = Some(v);
        self
    }

    pub fn span_columns(mut self, v: Span) -> StyleBuilder {
        self.span_columns = Some(v);
        self
    }

    pub fn align_self_h(mut self, v: Align) -> StyleBuilder {
        self.align_self_h = Some(v);
        self
    }

    pub fn align_self_v(mut self, v: Align) -> StyleBuilder {
        self.align_self_v = Some(v);
        self
    }

    /* layout */

    pub fn layout_overflow_x(mut self, v: LayoutOverflow) -> StyleBuilder {
        self.layout_overflow_x = Some(v);
        self
    }

    pub fn layout_overflow_y(mut self, v: LayoutOverflow) -> StyleBuilder {
        self.layout_overflow_y = Some(v);
        self
    }

    pub fn layout_overflow(mut self, v: LayoutOverflow) -> StyleBuilder {
        self.layout_overflow_x = Some(v);
        self.layout_overflow_y = Some(v);
        self
    }

    pub fn layout_flow(mut self, v: LayoutFlow) -> StyleBuilder {
        self.layout_flow = Some(v);
        self
    }

    pub fn layout_order(mut self, v: LayoutOrder) -> StyleBuilder {
        self.layout_order = Some(v);
        self
    }

    pub fn layout_gap_h(mut self, v: Scalar) -> StyleBuilder {
        self.layout_gap_h = Some(v);
        self
    }

    pub fn layout_gap_v(mut self, v: Scalar) -> StyleBuilder {
        self.layout_gap_v = Some(v);
        self
    }

    pub fn layout_align_h(mut self, v: Align) -> StyleBuilder {
        self.layout_align_h = Some(v);
        self
    }

    pub fn layout_align_v(mut self, v: Align) -> StyleBuilder {
        self.layout_align_v = Some(v);
        self
    }

    pub fn layout_rows(mut self, v: Vec<SizePolicy>) -> StyleBuilder {
        self.layout_rows = Some(v);
        self
    }

    pub fn layout_columns(mut self, v: Vec<SizePolicy>) -> StyleBuilder {
        self.layout_columns = Some(v);
        self
    }
    /* font */

    pub fn font_family(mut self, v: FontFamily) -> StyleBuilder {
        self.font_family = Some(v);
        self
    }

    pub fn font_stretch(mut self, v: FontStretch) -> StyleBuilder {
        self.font_stretch = Some(v);
        self
    }

    pub fn font_style(mut self, v: FontModifier) -> StyleBuilder {
        self.font_style = Some(v);
        self
    }

    pub fn font_weight(mut self, v: u16) -> StyleBuilder {
        self.font_weight = Some(v);
        self
    }

    /* text */
    pub fn justify(mut self, v: bool) -> StyleBuilder {
        self.justify = Some(v);
        self
    }

    pub fn letter_spacing(mut self, v: Scalar) -> StyleBuilder {
        self.letter_spacing = Some(v);
        self
    }

    pub fn line_spacing(mut self, v: Scalar) -> StyleBuilder {
        self.line_spacing = Some(v);
        self
    }

    pub fn paragraph_spacing(mut self, v: Scalar) -> StyleBuilder {
        self.paragraph_spacing = Some(v);
        self
    }

    pub fn whitespace_width(mut self, v: Scalar) -> StyleBuilder {
        self.whitespace_width = Some(v);
        self
    }

    pub fn tab_width(mut self, v: Scalar) -> StyleBuilder {
        self.tab_width = Some(v);
        self
    }

    pub fn text_overflow(mut self, v: TextOverflow) -> StyleBuilder {
        self.text_overflow = Some(v);
        self
    }

    pub fn text_wrap_mode(mut self, v: TextWrap) -> StyleBuilder {
        self.text_wrap_mode = Some(v);
        self
    }

    pub fn text_color(mut self, v: Color) -> StyleBuilder {
        self.text_color = Some(v);
        self
    }

    pub fn underline(mut self, v: Option<Color>) -> StyleBuilder {
        self.underline = Some(v);
        self
    }

    pub fn strikethrough(mut self, v: Option<Color>) -> StyleBuilder {
        self.strikethrough = Some(v);
        self
    }

    /* actions */

    pub fn build(&self) -> Style {
        Style {
            detect_cursor: self.detect_cursor.unwrap_or_default(),
            capture_absolute: self.detect_cursor.unwrap_or_default(),
            select_mode: self.select_mode.unwrap_or_default(),
            focus: self.focus.unwrap_or_default(),
            cursor: self.cursor.unwrap_or_default(),
            position: self.position.unwrap_or_default(),
            layer: self.layer.unwrap_or_default(),
            opacity: self.opacity.unwrap_or_default(),

            spacing_top: self.spacing_top.unwrap_or_default(),
            spacing_right: self.spacing_right.unwrap_or_default(),
            spacing_bottom: self.spacing_bottom.unwrap_or_default(),
            spacing_left: self.spacing_left.unwrap_or_default(),

            border_kind_top: self.border_kind_top.unwrap_or_default(),
            border_kind_right: self.border_kind_right.unwrap_or_default(),
            border_kind_bottom: self.border_kind_bottom.unwrap_or_default(),
            border_kind_left: self.border_kind_left.unwrap_or_default(),
            border_color_top: self.border_color_top.unwrap_or_default(),
            border_color_right: self.border_color_right.unwrap_or_default(),
            border_color_bottom: self.border_color_bottom.unwrap_or_default(),
            border_color_left: self.border_color_left.unwrap_or_default(),
            border_width_top: self.border_width_top.unwrap_or_default(),
            border_width_right: self.border_width_right.unwrap_or_default(),
            border_width_bottom: self.border_width_bottom.unwrap_or_default(),
            border_width_left: self.border_width_left.unwrap_or_default(),
            border_radius_upper_left: self.border_radius_upper_left.unwrap_or_default(),
            border_radius_upper_right: self.border_radius_upper_right.unwrap_or_default(),
            border_radius_lower_right: self.border_radius_lower_right.unwrap_or_default(),
            border_radius_lower_left: self.border_radius_lower_left.unwrap_or_default(),

            background_fill: self.background_fill.clone().unwrap_or_default(),

            span_rows: self.span_rows.unwrap_or_default(),
            span_columns: self.span_columns.unwrap_or_default(),
            align_self_h: self.align_self_h.unwrap_or_default(),
            align_self_v: self.align_self_v.unwrap_or_default(),

            layout_overflow_x: self.layout_overflow_x.unwrap_or_default(),
            layout_overflow_y: self.layout_overflow_y.unwrap_or_default(),
            layout_flow: self.layout_flow.unwrap_or_default(),
            layout_order: self.layout_order.unwrap_or_default(),
            layout_gap_h: self.layout_gap_h.unwrap_or_default(),
            layout_gap_v: self.layout_gap_v.unwrap_or_default(),
            layout_align_h: self.layout_align_h.unwrap_or_default(),
            layout_align_v: self.layout_align_v.unwrap_or_default(),
            layout_rows: self.layout_rows.clone().unwrap_or_default(),
            layout_columns: self.layout_columns.clone().unwrap_or_default(),

            font_family: self.font_family.clone().unwrap_or_default(),
            font_stretch: self.font_stretch.unwrap_or_default(),
            font_style: self.font_style.unwrap_or_default(),
            font_weight: self.font_weight.unwrap_or_default(),

            justify: self.justify.unwrap_or_default(),
            letter_spacing: self.letter_spacing.unwrap_or_default(),
            line_spacing: self.line_spacing.unwrap_or_default(),
            paragraph_spacing: self.paragraph_spacing.unwrap_or_default(),
            whitespace_width: self.whitespace_width.unwrap_or_default(),
            tab_width: self.tab_width.unwrap_or_default(),
            text_overflow: self.text_overflow.unwrap_or_default(),
            text_wrap_mode: self.text_wrap_mode.unwrap_or_default(),
            text_color: self.text_color.unwrap_or_default(),
            underline: self.underline.unwrap_or_default(),
            strikethrough: self.strikethrough.unwrap_or_default(),
        }
    }
}

impl Add for StyleBuilder {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            detect_cursor:             rhs.detect_cursor.or_else(|| self.detect_cursor),
            capture_absolute:          rhs.capture_absolute.or_else(|| self.capture_absolute),
            select_mode:               rhs.select_mode.or_else(|| self.select_mode),
            focus:                     rhs.focus.or_else(|| self.focus),
            cursor:                    rhs.cursor.or_else(|| self.cursor),
            position:                  rhs.position.or_else(|| self.position),
            layer:                     rhs.layer.or_else(|| self.layer),
            opacity:                   rhs.opacity.or_else(|| self.opacity),

            spacing_top:               rhs.spacing_top.or_else(|| self.spacing_top),
            spacing_right:             rhs.spacing_right.or_else(|| self.spacing_right),
            spacing_bottom:            rhs.spacing_bottom.or_else(|| self.spacing_bottom),
            spacing_left:              rhs.spacing_left.or_else(|| self.spacing_left),

            border_kind_top:           rhs.border_kind_top.or_else(|| self.border_kind_top),
            border_kind_right:         rhs.border_kind_right.or_else(|| self.border_kind_right),
            border_kind_bottom:        rhs.border_kind_bottom.or_else(|| self.border_kind_bottom),
            border_kind_left:          rhs.border_kind_left.or_else(|| self.border_kind_left),
            border_color_top:          rhs.border_color_top.or_else(|| self.border_color_top),
            border_color_right:        rhs.border_color_right.or_else(|| self.border_color_right),
            border_color_bottom:       rhs.border_color_bottom.or_else(|| self.border_color_bottom),
            border_color_left:         rhs.border_color_left.or_else(|| self.border_color_left),
            border_width_top:          rhs.border_width_top.or_else(|| self.border_width_top),
            border_width_right:        rhs.border_width_right.or_else(|| self.border_width_right),
            border_width_bottom:       rhs.border_width_bottom.or_else(|| self.border_width_bottom),
            border_width_left:         rhs.border_width_left.or_else(|| self.border_width_left),
            border_radius_upper_left:  rhs.border_radius_upper_left.or_else(|| self.border_radius_upper_left),
            border_radius_upper_right: rhs.border_radius_upper_right.or_else(|| self.border_radius_upper_right),
            border_radius_lower_right: rhs.border_radius_lower_right.or_else(|| self.border_radius_lower_right),
            border_radius_lower_left:  rhs.border_radius_lower_left.or_else(|| self.border_radius_lower_left),

            background_fill:           rhs.background_fill.or_else(|| self.background_fill.clone()),

            span_rows:                 rhs.span_rows.or_else(|| self.span_rows.clone()),
            span_columns:              rhs.span_columns.or_else(|| self.span_columns),
            align_self_h:              rhs.align_self_h.or_else(|| self.align_self_h),
            align_self_v:              rhs.align_self_v.or_else(|| self.align_self_v),

            layout_overflow_x:         rhs.layout_overflow_x.or_else(|| self.layout_overflow_x),
            layout_overflow_y:         rhs.layout_overflow_y.or_else(|| self.layout_overflow_y),
            layout_flow:               rhs.layout_flow.or_else(|| self.layout_flow),
            layout_order:              rhs.layout_order.or_else(|| self.layout_order),
            layout_gap_h:              rhs.layout_gap_h.or_else(|| self.layout_gap_h),
            layout_gap_v:              rhs.layout_gap_v.or_else(|| self.layout_gap_v),
            layout_align_h:            rhs.layout_align_h.or_else(|| self.layout_align_h),
            layout_align_v:            rhs.layout_align_v.or_else(|| self.layout_align_v),
            layout_rows:               rhs.layout_rows.or_else(|| self.layout_rows.clone()),
            layout_columns:            rhs.layout_columns.or_else(|| self.layout_columns.clone()),

            font_family:               rhs.font_family.or_else(|| self.font_family.clone()),
            font_stretch:              rhs.font_stretch.or_else(|| self.font_stretch.clone()),
            font_style:                rhs.font_style.or_else(|| self.font_style),
            font_weight:               rhs.font_weight.or_else(|| self.font_weight),

            justify:                   rhs.justify.or_else(|| self.justify),
            letter_spacing:            rhs.letter_spacing.or_else(|| self.letter_spacing),
            line_spacing:              rhs.line_spacing.or_else(|| self.line_spacing),
            paragraph_spacing:         rhs.paragraph_spacing.or_else(|| self.paragraph_spacing),
            whitespace_width:          rhs.whitespace_width.or_else(|| self.whitespace_width),
            tab_width:                 rhs.tab_width.or_else(|| self.tab_width),
            text_overflow:             rhs.text_overflow.or_else(|| self.text_overflow),
            text_wrap_mode:            rhs.text_wrap_mode.or_else(|| self.text_wrap_mode),
            text_color:                rhs.text_color.or_else(|| self.text_color),
            underline:                 rhs.underline.or_else(|| self.underline),
            strikethrough:             rhs.strikethrough.or_else(|| self.strikethrough),
        }
    }
}

#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub enum Size {
    Percent(u8),
    Fixed(u32),
}

#[derive(Clone)]
pub enum Align {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub enum ContentWrap {
    Wrap,
    NoWrap,
}

#[derive(Clone)]
pub enum Style {
    Orientation(Orientation),
    MinSize(Size),
    MaxSize(Size),
    ContentWrap(ContentWrap),
    HAlign(Align),
    VAlign(Align),
    Margin(Size, Size, Size, Size),
}
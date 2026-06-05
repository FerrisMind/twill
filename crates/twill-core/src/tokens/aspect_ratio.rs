/// AspectRatio token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectRatio {
    Auto,
    Square,
    Video,
    Custom(u16, u16),
}

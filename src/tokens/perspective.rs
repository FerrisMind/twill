//! Perspective tokens from Tailwind v4 theme scale.

/// Perspective tokens (`--perspective-*` in Tailwind v4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Perspective {
    Dramatic,
    Near,
    Normal,
    Midrange,
    Distant,
    CustomPx(u16),
}

impl Perspective {
    /// Value in pixels.
    pub fn to_px(&self) -> u16 {
        match self {
            Perspective::Dramatic => 100,
            Perspective::Near => 300,
            Perspective::Normal => 500,
            Perspective::Midrange => 800,
            Perspective::Distant => 1200,
            Perspective::CustomPx(value) => *value,
        }
    }
}

impl Perspective {
    pub fn value(&self) -> String {
        format!("{}px", self.to_px())
    }
}

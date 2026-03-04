//! Perspective tokens for 3D transform depth.

/// Perspective tokens.
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

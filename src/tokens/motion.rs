//! Motion design tokens adapted from Tailwind CSS.

use crate::traits::ToCss;

/// Transition duration tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionDuration {
    Ms0,
    Ms75,
    Ms100,
    Ms150,
    Ms200,
    Ms300,
    Ms500,
    Ms700,
    Ms1000,
    CustomMs(u16),
}

impl TransitionDuration {
    pub fn as_millis(&self) -> u16 {
        match self {
            Self::Ms0 => 0,
            Self::Ms75 => 75,
            Self::Ms100 => 100,
            Self::Ms150 => 150,
            Self::Ms200 => 200,
            Self::Ms300 => 300,
            Self::Ms500 => 500,
            Self::Ms700 => 700,
            Self::Ms1000 => 1000,
            Self::CustomMs(v) => *v,
        }
    }
}

impl ToCss for TransitionDuration {
    fn to_css(&self) -> String {
        format!("{}ms", self.as_millis())
    }
}

/// Timing function tokens (`--ease-*` in Tailwind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Easing {
    Linear,
    In,
    Out,
    InOut,
}

impl ToCss for Easing {
    fn to_css(&self) -> String {
        match self {
            Self::Linear => "linear".to_string(),
            Self::In => "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            Self::Out => "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            Self::InOut => "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
        }
    }
}

/// Built-in animation tokens (`--animate-*` in Tailwind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationToken {
    None,
    Spin,
    Ping,
    Pulse,
    Bounce,
}

impl ToCss for AnimationToken {
    fn to_css(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Spin => "spin 1s linear infinite".to_string(),
            Self::Ping => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            Self::Pulse => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            Self::Bounce => "bounce 1s infinite".to_string(),
        }
    }
}

/// Tailwind-compatible default transition tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MotionDefaults {
    pub duration: TransitionDuration,
    pub easing: Easing,
}

impl Default for MotionDefaults {
    fn default() -> Self {
        Self {
            duration: TransitionDuration::Ms150,
            easing: Easing::InOut,
        }
    }
}

/// Transition property tokens (`transition-*` in Tailwind).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransitionProperty {
    None,
    All,
    Default,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Custom(String),
}

impl ToCss for TransitionProperty {
    fn to_css(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::All => "all".to_string(),
            Self::Default => "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".to_string(),
            Self::Colors => "color, background-color, border-color, text-decoration-color, fill, stroke".to_string(),
            Self::Opacity => "opacity".to_string(),
            Self::Shadow => "box-shadow".to_string(),
            Self::Transform => "transform".to_string(),
            Self::Custom(val) => val.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_css() {
        assert_eq!(TransitionDuration::Ms150.to_css(), "150ms");
        assert_eq!(TransitionDuration::CustomMs(350).to_css(), "350ms");
    }

    #[test]
    fn test_easing_css() {
        assert_eq!(Easing::In.to_css(), "cubic-bezier(0.4, 0, 1, 1)");
    }
}

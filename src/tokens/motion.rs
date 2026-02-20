//! Motion design tokens adapted from Tailwind.

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

    pub fn value(&self) -> String {
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

impl Easing {
    pub fn value(&self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::In => "cubic-bezier(0.4, 0, 1, 1)",
            Self::Out => "cubic-bezier(0, 0, 0.2, 1)",
            Self::InOut => "cubic-bezier(0.4, 0, 0.2, 1)",
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

impl AnimationToken {
    pub fn value(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Spin => "spin 1s linear infinite",
            Self::Ping => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
            Self::Pulse => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
            Self::Bounce => "bounce 1s infinite",
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

impl TransitionProperty {
    pub fn value(&self) -> &str {
        match self {
            Self::None => "none",
            Self::All => "all",
            Self::Default => {
                "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter"
            }
            Self::Colors => {
                "color, background-color, border-color, text-decoration-color, fill, stroke"
            }
            Self::Opacity => "opacity",
            Self::Shadow => "box-shadow",
            Self::Transform => "transform",
            Self::Custom(val) => val.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_value() {
        assert_eq!(TransitionDuration::Ms150.value(), "150ms");
        assert_eq!(TransitionDuration::CustomMs(350).value(), "350ms");
    }

    #[test]
    fn test_easing_value() {
        assert_eq!(Easing::In.value(), "cubic-bezier(0.4, 0, 1, 1)");
    }
}

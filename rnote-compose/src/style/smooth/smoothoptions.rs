use crate::Color;
use crate::style::PressureProfile;

use serde::{Deserialize, Serialize};

/// Options for shapes that can be drawn smoothly (plain)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "smooth_options")]
pub struct SmoothOptions {
    /// The stroke width
    #[serde(rename = "stroke_width")]
    pub stroke_width: f64,
    /// The stroke color
    #[serde(rename = "stroke_color")]
    pub stroke_color: Option<Color>,
    /// The fill color
    #[serde(rename = "fill_color")]
    pub fill_color: Option<Color>,
    /// Pressure profile
    #[serde(rename = "pressure_profile")]
    pub pressure_profile: PressureProfile,
}

impl Default for SmoothOptions {
    fn default() -> Self {
        Self {
            stroke_width: Self::WIDTH_DEFAULT,
            stroke_color: Some(Color::BLACK),
            fill_color: None,
            pressure_profile: PressureProfile::Cbrt,
        }
    }
}

impl SmoothOptions {
    /// The default width
    pub const WIDTH_DEFAULT: f64 = 1.0;
    /// The min width
    pub const WIDTH_MIN: f64 = 0.1;
    /// The max width
    pub const WIDTH_MAX: f64 = 1000.0;
}

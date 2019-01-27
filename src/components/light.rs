use crate::constants::*;
use specs::prelude::*;
#[derive(Component, Debug, Clone)]
pub struct Light {
    pub radius: f32,
    pub color: raylib::Color,
}

impl Light {
    pub const fn new(radius: f32, color: raylib::Color) -> Self {
        Light { radius, color }
    }

    /// light power at a distance
    pub fn power_at(&self, d: f32) -> f32 {
        // ((10.0 / (1.0 / 10.0)) / (10.0 * 30.0))
        ((self.radius * (1.0 / LIGHT_FALLOFF)) / (LIGHT_FALLOFF * d * d))
    }
}

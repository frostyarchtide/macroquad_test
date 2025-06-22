use macroquad::{prelude::*, time};
use std::f32::consts::TAU;

pub struct Planet {
    pub radius: f32,
    pub color: Color,
    pub orbital_radius: f32,
    pub orbital_period: f32,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            radius: 1.,
            color: WHITE,
            orbital_radius: 0.,
            orbital_period: 1.,
        }
    }
}

impl Planet {
    pub fn new(
        radius: f32,
        color: Color,
        orbital_radius: f32,
        orbital_period: Option<f32>,
    ) -> Self {
        Self {
            radius,
            color,
            orbital_period: orbital_period.unwrap_or(orbital_radius * orbital_radius),
            orbital_radius,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        let angle = -(time::get_time() as f32 / self.orbital_period).fract() * TAU;
        let position = vec2(angle.cos(), angle.sin()) * self.orbital_radius;
        
        return position;
    }

    pub fn draw(&self) {
        let position = self.get_position();
        draw_poly(position.x, position.y, 64, self.radius, 0., self.color);
    }
}

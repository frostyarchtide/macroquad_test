use macroquad::{prelude::*, time};
use std::f32::consts::TAU;

pub struct Planet {
    pub radius: f32,
    pub color: Color,
    pub orbital_radius: f32,
    pub orbital_period: f32,
    pub position: Vec2,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            radius: 1.,
            color: WHITE,
            orbital_radius: 0.,
            orbital_period: 1.,
            position: Default::default(),
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
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let angle = -(time::get_time() as f32 / self.orbital_period).fract() * TAU;
        self.position = vec2(angle.cos(), angle.sin()) * self.orbital_radius;
    }

    pub fn draw(&self) {
        draw_poly(
            self.position.x,
            self.position.y,
            64,
            self.radius,
            0.,
            self.color,
        );
    }
}

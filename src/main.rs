use std::f32::consts::TAU;

use macroquad::{prelude::*, time};

struct Planet {
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
    fn new(radius: f32, color: Color, orbital_radius: f32, orbital_period: Option<f32>) -> Self {
        Self {
            radius,
            color,
            orbital_period: orbital_period.unwrap_or(orbital_radius * orbital_radius),
            orbital_radius,
        }
    }
}

#[macroquad::main("macroquad_test")]
async fn main() {
    let planets = vec![
        Planet::new(3., YELLOW, 0., Some(1.)),
        Planet::new(0.5, RED, 4., None),
        Planet::new(1., GREEN, 10., None),
    ];

    loop {
        clear_background(BLACK);

        let aspect_ratio = screen_width() as f32 / screen_height() as f32;
        let mut camera =
            Camera2D::from_display_rect(Rect::new(-aspect_ratio, -1., aspect_ratio * 2., 2.));

        camera.zoom = vec2(0.1 / aspect_ratio, 0.1);

        set_camera(&camera);

        for planet in planets.iter() {
            let angle = -(time::get_time() as f32 / planet.orbital_period).fract() * TAU;
            let position = vec2(angle.cos(), angle.sin()) * planet.orbital_radius;
            draw_poly(position.x, position.y, 64, planet.radius, 0., planet.color);
        }

        set_default_camera();

        next_frame().await;
    }
}

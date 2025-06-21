mod camera;
mod planet;

use crate::{camera::Camera, planet::*};
use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    let planets = vec![
        Planet::new(3., YELLOW, 0., Some(1.)),
        Planet::new(0.5, RED, 4., None),
        Planet::new(1., GREEN, 10., None),
    ];

    let mut camera = Camera::default();

    loop {
        clear_background(BLACK);

        camera.update();

        set_camera(camera.get_camera());

        for planet in planets.iter() {
            planet.draw();
        }

        set_default_camera();

        next_frame().await;
    }
}

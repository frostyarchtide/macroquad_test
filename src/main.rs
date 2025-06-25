mod camera;
mod planet;
mod target;

use crate::{camera::Camera, planet::*, target::*};
use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    let mut planets = vec![
        Planet::new(3., YELLOW, 0., Some(1.)),
        Planet::new(0.5, RED, 4., None),
        Planet::new(1., GREEN, 10., None),
    ];

    let mut target = Target::new();
    let mut camera = Camera::default();

    loop {
        clear_background(BLACK);

        set_camera(&camera.camera);

        for planet in planets.iter_mut() {
            planet.update();
        }

        target.update(&camera, &planets);
        camera.update(&target, &planets);

        for planet in planets.iter() {
            planet.draw();
        }

        set_default_camera();

        target.draw(&camera, &planets);

        next_frame().await;
    }
}

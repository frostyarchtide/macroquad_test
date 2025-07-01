mod camera;
mod planet;
mod target;

use crate::{camera::Camera, planet::*, target::*};
use macroquad::{prelude::*, ui::*};

#[macroquad::main("macroquad_test")]
async fn main() {
    let mut planets = vec![
        Planet::new(
            Info {
                website: String::from("GitHub"),
                username: String::from("frostyarchtide"),
            },
            3.,
            YELLOW,
            0.,
            Some(1.),
        ),
        Planet::new(
            Info {
                website: String::from(""),
                username: String::from(""),
            },
            0.5,
            RED,
            4.,
            None,
        ),
        Planet::new(
            Info {
                website: String::from(""),
                username: String::from(""),
            },
            1.,
            GREEN,
            10.,
            None,
        ),
    ];

    let mut target = Target::new();
    let mut camera = Camera::default();

    loop {
        clear_background(BLACK);

        for planet in planets.iter_mut() {
            planet.update();
        }

        target.update(&camera, &planets);
        camera.update(&target, &planets);

        set_camera(&camera.camera);

        for planet in planets.iter() {
            planet.draw();
        }

        set_default_camera();

        target.draw(&camera, &planets);

        if let Some(planet) = target.planet {
            let planet = &planets[planet];
            let font_size = (screen_width() / 16.).floor();
            let padding = font_size / 4.;
            draw_text(planet.info.website.as_str(), 0., font_size / 2. + padding / 2., font_size, WHITE);
            draw_text(planet.info.username.as_str(), 0., font_size + padding * 2., font_size, WHITE);
        }

        next_frame().await;
    }
}

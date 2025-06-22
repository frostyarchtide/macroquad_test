mod camera;
mod planet;
mod target;

use crate::{camera::Camera, planet::*};
use macroquad::prelude::*;

const SELECTION_DOUBLE_CLICK_TIME: f64 = 0.2;
const MINIMUM_CROSSHAIR_RADIUS: f32 = 25.;
const CROSSHAIR_RADIUS_PADDING: f32 = 5.;
const CROSSHAIR_GAP_PERCENT: f32 = 0.5;
const CROSSHAIR_THICKNESS: f32 = 3.;
const CROSSHAIR_COLOR: Color = WHITE;


#[macroquad::main("macroquad_test")]
async fn main() {
    let planets = vec![
        Planet::new(3., YELLOW, 0., Some(1.)),
        Planet::new(0.5, RED, 4., None),
        Planet::new(1., GREEN, 10., None),
    ];

    let mut selected: Option<&Planet> = None;
    let mut camera = Camera::default();
    let mut last_click = -SELECTION_DOUBLE_CLICK_TIME;

    loop {
        clear_background(BLACK);

        camera.update();

        set_camera(&camera.camera);

        if is_mouse_button_pressed(MouseButton::Left) {
            let time = get_time();

            if time - last_click < SELECTION_DOUBLE_CLICK_TIME {
                selected = None;

                for planet in planets.iter() {
                    let mouse_position = camera.camera.screen_to_world(Vec2::from(mouse_position()));
                    let delta = mouse_position - planet.get_position();
                    if delta.length() < planet.radius {
                        selected = Some(planet);

                        break;
                    }
                }
            }

            last_click = get_time();
        }

        if let Some(planet) = selected {
            camera.camera.target = planet.get_position();
        }

        for planet in planets.iter() {
            planet.draw();
        }

        set_default_camera();

        if let Some(planet) = selected {
            let planet_position = planet.get_position();
            let position = camera.camera.world_to_screen(planet_position);
            let edge_position = camera.camera.world_to_screen(planet_position + Vec2::new(planet.radius, 0.));
            let radius = (edge_position.x - position.x + CROSSHAIR_RADIUS_PADDING).max(MINIMUM_CROSSHAIR_RADIUS);
            //draw_crosshair(position.x, position.y, radius, CROSSHAIR_GAP_PERCENT, CROSSHAIR_THICKNESS, CROSSHAIR_COLOR);
        }

        next_frame().await;
    }
}

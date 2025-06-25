use crate::{camera::Camera, planet::*};
use macroquad::prelude::*;

const DOUBLE_CLICK_TIME: f64 = 0.2;
const MINIMUM_RADIUS: f32 = 25.;
const RADIUS_PADDING: f32 = 5.;
const GAP_PERCENT: f32 = 0.5;
const THICKNESS: f32 = 3.;
const COLOR: Color = WHITE;

pub struct Target {
    pub planet: Option<usize>,
    pub last_click: f64,
}

impl Target {
    pub fn new() -> Self {
        Self {
            planet: Default::default(),
            last_click: -DOUBLE_CLICK_TIME,
        }
    }

    pub fn update(&mut self, camera: &Camera, planets: &Vec<Planet>) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let time = get_time();

            if time - self.last_click < DOUBLE_CLICK_TIME {
                self.planet = None;

                for (i, planet) in planets.iter().enumerate() {
                    let mouse_position =
                        camera.camera.screen_to_world(Vec2::from(mouse_position()));
                    let delta = mouse_position - planet.position;
                    if delta.length() < planet.radius {
                        self.planet = Some(i);

                        break;
                    }
                }
            }

            self.last_click = get_time();
        }
    }

    pub fn draw(&self, camera: &Camera, planets: &Vec<Planet>) {
        if let Some(i) = self.planet {
            let planet = &planets[i];
            let screen_radius = camera
                .camera
                .world_to_screen(planet.position + vec2(planet.radius, 0.))
                .x
                - camera.camera.world_to_screen(planet.position).x;
            let radius = screen_radius.max(MINIMUM_RADIUS) + RADIUS_PADDING;
            let thick_radius = radius + THICKNESS / 2.;
            let gap_radius = radius * GAP_PERCENT;
            let position = camera.camera.world_to_screen(planet.position);

            draw_line(
                position.x - thick_radius,
                position.y - radius,
                position.x - gap_radius,
                position.y - radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x - radius,
                position.y - thick_radius,
                position.x - radius,
                position.y - gap_radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x + thick_radius,
                position.y - radius,
                position.x + gap_radius,
                position.y - radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x + radius,
                position.y - thick_radius,
                position.x + radius,
                position.y - gap_radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x - thick_radius,
                position.y + radius,
                position.x - gap_radius,
                position.y + radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x - radius,
                position.y + thick_radius,
                position.x - radius,
                position.y + gap_radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x + thick_radius,
                position.y + radius,
                position.x + gap_radius,
                position.y + radius,
                THICKNESS,
                COLOR,
            );
            draw_line(
                position.x + radius,
                position.y + thick_radius,
                position.x + radius,
                position.y + gap_radius,
                THICKNESS,
                COLOR,
            );
        }
    }
}

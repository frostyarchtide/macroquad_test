use macroquad::prelude::*;
use std::ops::RangeInclusive;

const ZOOM_SENSITIVITY: f32 = 0.005;
const ZOOM_RANGE: RangeInclusive<f32> = 0.05..=2.;

pub struct Camera {
    pub camera: Camera2D,
    zoom: f32,
    mouse_pan_position: Option<Vec2>,
    mouse_zoom_y: Option<f32>,
    touch_pan_position: Option<Vec2>,
    pinch_distance: Option<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            zoom: 0.1,
            mouse_pan_position: Default::default(),
            mouse_zoom_y: Default::default(),
            touch_pan_position: Default::default(),
            pinch_distance: Default::default(),
        }
    }
}

impl Camera {
    pub fn update(&mut self) {
        let mouse_position = Vec2::from(mouse_position());
        let touches = touches();

        if touches.len() == 0 && self.mouse_zoom_y.is_none() && is_mouse_button_down(MouseButton::Left) {
            if let Some(mouse_pan_position) = self.mouse_pan_position {
                let delta = self.camera.screen_to_world(mouse_position) - self.camera.screen_to_world(mouse_pan_position);
                self.camera.target -= delta;
            }

            self.mouse_pan_position = Some(mouse_position);
        } else {
            self.mouse_pan_position = None;
        }

        if touches.len() == 0 && self.mouse_pan_position.is_none() && is_mouse_button_down(MouseButton::Right) {
            if let Some(mouse_zoom_y) = self.mouse_zoom_y {
                let delta_y = mouse_position.y - mouse_zoom_y;
                let scale = 1.0 - delta_y * ZOOM_SENSITIVITY;
                self.zoom = (self.zoom * scale).clamp(*ZOOM_RANGE.start(), *ZOOM_RANGE.end());
            }

            self.mouse_zoom_y = Some(mouse_position.y);
        } else {
            self.mouse_zoom_y = None;
        }

        if touches.len() == 1 {
            let touch = &touches[0];

            if let Some(touch_pan_position) = self.touch_pan_position {
                let delta = self.camera.screen_to_world(touch.position) - self.camera.screen_to_world(touch_pan_position);
                self.camera.target -= delta;
            }

            self.touch_pan_position = Some(touch.position);
        } else {
            self.touch_pan_position = None;
        }

        if touches.len() == 2 {
            let touch_1 = &touches[0];
            let touch_2 = &touches[1];
            let distance = touch_1.position.distance(touch_2.position);

            if let Some(pinch_distance) = self.pinch_distance {
                let scale = distance / pinch_distance;
                self.zoom = (self.zoom * scale).clamp(*ZOOM_RANGE.start(), *ZOOM_RANGE.end());
            }

            self.pinch_distance = Some(distance);
        } else {
            self.pinch_distance = None;
        }

        let (screen_width, screen_height) = (screen_width(), screen_height());
        let aspect_ratio = screen_width / screen_height;
        self.camera.zoom = vec2(self.zoom / aspect_ratio, self.zoom);
    }
}

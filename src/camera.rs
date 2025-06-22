use macroquad::prelude::*;

const ZOOM_SENSITIVITY: f32 = 0.005;

pub struct Camera {
    camera: Camera2D,
    zoom: f32,
    last_mouse_position: Vec2,
    last_touch_position: Option<Vec2>,
    last_pinch_distance: Option<f32>,
    panning: bool,
    zooming: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            zoom: 0.1,
            last_mouse_position: Default::default(),
            last_touch_position: Default::default(),
            last_pinch_distance: Default::default(),
            panning: false,
            zooming: false,
        }
    }
}

impl Camera {
    pub fn get_camera(&self) -> &Camera2D {
        return &self.camera;
    }

    pub fn update(&mut self) {
        let mouse_position = Vec2::from(mouse_position());
        let touches = touches();

        if touches.len() == 0 && is_mouse_button_down(MouseButton::Left) {
            let delta = self.camera.screen_to_world(mouse_position) - self.camera.screen_to_world(self.last_mouse_position);
            self.camera.target -= delta;
        }

        if touches.len() == 0 && is_mouse_button_down(MouseButton::Right) {
            let delta_y = mouse_position.y - self.last_mouse_position.y;
            let scale = 1.0 - delta_y * ZOOM_SENSITIVITY;
            self.zoom *= scale;
        }

        self.last_mouse_position = mouse_position;

        if touches.len() == 1 {
            let touch = &touches[0];

            if let Some(last_touch_position) = self.last_touch_position {
                let delta = self.camera.screen_to_world(touch.position) - self.camera.screen_to_world(last_touch_position);
                self.camera.target -= delta;
            }

            self.last_touch_position = Some(touch.position);
        } else {
            self.last_touch_position = None;
        }

        if touches.len() == 2 {
            let touch_1 = &touches[0];
            let touch_2 = &touches[1];

            let pinch_distance = touch_1.position.distance(touch_2.position);

            if let Some(last_pinch_distance) = self.last_pinch_distance {
                let scale = pinch_distance / last_pinch_distance;
                self.zoom *= scale;
            }

            self.last_pinch_distance = Some(pinch_distance);
        } else {
            self.last_pinch_distance = None;
        }

        let (screen_width, screen_height) = (screen_width(), screen_height());
        let aspect_ratio = screen_width / screen_height;
        self.camera.zoom = vec2(self.zoom / aspect_ratio, self.zoom);
    }
}

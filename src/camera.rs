use macroquad::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.002;
const SCROLL_SENSITIVITY: f32 = 0.005;

pub struct Camera {
    camera: Camera2D,
    zoom: f32,
    last_mouse_position: Vec2,
    last_touch_position: Vec2,
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
    pub fn new(
        camera: Camera2D,
        zoom: f32,
        last_mouse_position: Vec2,
        last_touch_position: Vec2,
        last_pinch_distance: Option<f32>,
    ) -> Self {
        Self {
            camera,
            zoom,
            last_mouse_position,
            last_touch_position,
            last_pinch_distance,
            ..Default::default()
        }
    }

    pub fn get_camera(&self) -> &Camera2D {
        return &self.camera;
    }

    pub fn update(&mut self) {
        let mouse_position = Vec2::from(mouse_position());
        let touches = touches();

        if touches.len() == 0 && is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_position - self.last_mouse_position;
            self.camera.target -= delta * MOUSE_SENSITIVITY / self.zoom;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let delta_y = mouse_position.y - self.last_mouse_position.y;
            let scale = 1.0 - delta_y * SCROLL_SENSITIVITY;
            self.zoom *= scale;
        }

        self.last_mouse_position = mouse_position;

        if touches.len() == 2 {
            let touch_1 = &touches[0];
            let touch_2 = &touches[1];

            let pinch_distance = touch_1.position.distance(touch_2.position);

            if let Some(last_pinch_distance) = self.last_pinch_distance {
                let scale = pinch_distance / last_pinch_distance;
                self.zoom *= scale;
            }

            self.last_pinch_distance = Some(pinch_distance);
        }

        let (screen_width, screen_height) = (screen_width(), screen_height());
        let aspect_ratio = screen_width / screen_height;
        self.camera.zoom = vec2(self.zoom / aspect_ratio, self.zoom);
    }
}

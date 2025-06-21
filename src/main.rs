use macroquad::prelude::*;

#[macroquad::main("macroquad_test")]
async fn main() {
    let fragment_shader = FRAGMENT_SHADER.to_string();
    let vertex_shader = VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader,
            fragment: &fragment_shader,
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    let camera = Camera2D::default();

    loop {
        clear_background(WHITE);

        set_camera(&camera);

        gl_use_material(&material);
        draw_rectangle(-1., -1., 2., 2., WHITE);
        gl_use_default_material();

        set_default_camera();

        next_frame().await;
    }
}

const FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

void main() {
    gl_FragColor = vec4(uv, 0.0, 1.0);
}
";

const VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";

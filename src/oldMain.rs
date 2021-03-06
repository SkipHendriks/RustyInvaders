#[macro_use]
extern crate glium;
extern crate stopwatch;
extern crate image;

use std::io::Cursor;
use stopwatch::{Stopwatch};
mod game;
mod player;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let image = image::load(Cursor::new(&include_bytes!("./img/ship.png")[..]),
        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();


    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 2.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [2.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;


        uniform mat4 pos_matrix;
        uniform mat4 rot_matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = pos_matrix * rot_matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;


        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let mut t: f32 = -0.5;

    let mut r: f32 = 0.2;

    loop {

        let sw = Stopwatch::start_new();

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        r += 0.0002;
        if r > 0.5 {
            r = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let uniforms = uniform! {
            pos_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            rot_matrix: [
                [ r.cos(), r.sin(), 0.0, 0.0],
                [-r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }

        let time = sw.elapsed_ms();

        //println!("Ms: {}", 1000 / time);

    }
}

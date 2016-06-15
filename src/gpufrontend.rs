use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::backend::glutin_backend::GlutinFacade;

extern crate glium;

use game::{Vertex};

pub struct GpuFrontend {
    display: GlutinFacade,
    program: Program,
    indices: glium::index::NoIndices,
}

impl GpuFrontend {
    pub fn new() -> GpuFrontend {
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

        let new_display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let new_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let new_program = glium::Program::from_source(&new_display, vertex_shader_src, fragment_shader_src, None).unwrap();

        GpuFrontend {
            display: new_display,
            indices: new_indices,
            program: new_program,
        }
    }

    pub fn draw(&self, vertex_buffer: &glium::VertexBuffer<Vertex>, position: f32, rotation: f32, texture: &glium::Texture2d) {
        println!("rot: {}  pos: {}", rotation, position);

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            pos_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ position , 0.0, 0.0, 1.0f32],
            ],
            rot_matrix: [
                [ rotation.cos(), rotation.sin(), 0.0, 0.0],
                [-rotation.sin(), rotation.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: texture,
        };

        target.draw(vertex_buffer, indices, &self.program, &uniforms,
                &Default::default()).unwrap();
        target.finish().unwrap();
        println!("finished");
    }
}
use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use glium::backend::glutin_backend::GlutinFacade;

extern crate glium;

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
}
extern crate image;
extern crate glium;

use std::io::Cursor;

use game::{Renderable,Vertex};

pub use glium::backend::glutin_backend::GlutinFacade as Display;


pub struct Player {
    pub position: f32,
    pub rotation: f32,
    pub texture: glium::Texture2d,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
}

impl Player {
    pub fn new(display : &Display) -> Player {
        // Setup texture
        let image = image::load(Cursor::new(&include_bytes!("./img/ship.png")[..]),
            image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let new_texture = glium::texture::Texture2d::new(display, image).unwrap();

        implement_vertex!(Vertex, position, tex_coords);


        // Setup vertexBuffer
        let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
        let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 2.0] };
        let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [2.0, 0.0] };
        let new_shape = vec![vertex1, vertex2, vertex3];

        let new_vertex_buffer = glium::VertexBuffer::new(display, &new_shape).unwrap();
    
        Player {
            position: 0.0,
            rotation: 0.0,
            texture: new_texture,
            vertex_buffer: new_vertex_buffer,
        }
    }

    pub fn updatePosition(&self) {
        let mut mut_pos = self.position;
        let mut mut_rot = self.rotation;



        mut_pos += 0.0002;
        if mut_pos > 0.5 {
            mut_pos = -0.5;
        }

        mut_rot += 0.0002;
        if mut_rot > 0.5 {
            mut_rot = -0.5;
        }
    }
}

impl Renderable for Player {
    fn get_render_info(&mut self) -> (&glium::VertexBuffer<Vertex>, f32, f32, &glium::Texture2d) {
        (&self.vertex_buffer, self.position, self.rotation, &self.texture)
    }
}
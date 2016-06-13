extern crate image;
extern crate glium;

use std::io::Cursor;

pub use glium::backend::glutin_backend::GlutinFacade as Display;


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct Player {
    pub position: f64,
    pub rotation: f64,
    pub texture: glium::Texture2d,
    pub vertexBuffer: glium::VertexBuffer<Vertex>,
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
        let shape = vec![vertex1, vertex2, vertex3];

        let newVertexBuffer = glium::VertexBuffer::new(display, &shape).unwrap();

        // match newVertexBuffer {
        //     Ok(_) => println!("yo"),
        //     Err(_) => println!("error"),
        // }

    

        Player {
            position: 0.0,
            rotation: 0.0,
            texture: new_texture,
            vertexBuffer: newVertexBuffer,
        }
    }
}
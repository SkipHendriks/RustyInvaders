extern crate image;
extern crate glium;

use std::io::Cursor;

pub use glium::backend::glutin_backend::GlutinFacade as Display;


pub struct Player {
    pub position: f64,
    pub rotation: f64,
    pub texture: glium::Texture2d,
}

impl Player {
    pub fn new(display : &Display) -> Player {
        let image = image::load(Cursor::new(&include_bytes!("./img/ship.png")[..]),
            image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let new_texture = glium::texture::Texture2d::new(display, image).unwrap();


        Player {
            position: 0.0,
            rotation: 0.0,
            texture: new_texture,
        }
    }
}
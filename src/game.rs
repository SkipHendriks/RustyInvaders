extern crate glium;

pub use glium::backend::glutin_backend::GlutinFacade as Display;

use player;
use gpufrontend;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}


pub trait Renderable {
    fn get_render_info(&mut self) -> (&glium::VertexBuffer<Vertex>, f32, f32, &glium::Texture2d);
}

pub struct Game {
    elapsed_time: i64,
    player: player::Player,
    gpu_frontend: gpufrontend::GpuFrontend, // entities: Vec<Box<Renderable>>,
}

impl Game {
    // Constructor
    pub fn new() -> Game {
        use glium::DisplayBuild;

        let new_display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let new_player = player::Player::new(&new_display);
        let new_gpu_frontend = gpufrontend::GpuFrontend::new();
        // let new_entities = vec![new_player];


        Game {
            elapsed_time: 0,
            player: new_player,
            gpu_frontend: new_gpu_frontend, // entities: new_entities,
        }
    }


    // Main method that gets called during every iteration of the owners loop
    pub fn tick(&self, time: i64) {
        let mut elapsedTime = self.elapsed_time;

        elapsedTime += time;
        self.player.updatePosition();

    }

    // Render method that is called to display the contents of the game
    pub fn render(&mut self) {
        let (vertex_buffer, position, rotation, texture) = self.player.get_render_info();
        self.gpu_frontend.draw(&vertex_buffer, position, rotation, &texture);
    }
}
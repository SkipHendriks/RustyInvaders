extern crate glium;

pub use glium::backend::glutin_backend::GlutinFacade as Display;

use player;


trait Renderable {
    fn render(&self, &Display) -> &Display;
}

pub struct Game {
    elapsedTime: i64,
    player: player::Player,
    display: Display,
    //entities: Vec<Box<Renderable>>,
}

impl Game {
    // Constructor
    pub fn new() -> Game {
        use glium::{DisplayBuild};

        let new_display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let new_player = player::Player::new(&new_display);
        //let new_entities = vec![new_player];
        

        Game {
            elapsedTime: 0,
            player: new_player,
            display: new_display,
            //entities: new_entities,
        }
    }

    // Main method that gets called during every iteration of the owners loop
    pub fn tick(&self, time: i64){
        let mut elapsedTime = self.elapsedTime;

        elapsedTime += time;

    }

    // Render method that is called to display the contents of the game
    pub fn render(&self){

    }
}
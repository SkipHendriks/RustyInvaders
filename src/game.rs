extern crate glium;

use player;

struct Game {
    elapsedTime: f64,
    player: player::Player,
    //display: glium::glutin::Window,
}


impl Game {
    // Constructor
    pub fn new() -> Game {
        let new_player = player::Player::new();

        Game {
            elapsedTime: 0.0,
            player: new_player,
        }
    }

    // Main method that gets called during every iteration of the owners loop
    pub fn tick(&self, time: f64){
        let mut elapsedTime = self.elapsedTime;

        elapsedTime += time;

    }
}
#[macro_use]
extern crate glium;
extern crate stopwatch;

use stopwatch::{Stopwatch};

mod game;
mod player;
mod gpufrontend;

fn main() {
   let game = game::Game::new();
   let mut stopwatch = Stopwatch::start_new();

   loop {
       game.render();
       game.tick(stopwatch.elapsed_ms());
       stopwatch.restart();
   }
}

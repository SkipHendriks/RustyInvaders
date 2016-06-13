#[macro_use]
extern crate glium;
extern crate stopwatch;

use stopwatch::{Stopwatch};
mod game;
mod player;

fn main() {
   let game = game::Game::new();
   let stopwatch = Stopwatch::new();

   loop {
       let sw = Stopwatch::start_new();

       game.render();
       game.tick(sw.elapsed_ms());
   }
}
use std::error::Error;

use miniquad::EventHandler;

use crate::gl::GraphicsLibrary;

pub struct Game {
    pub(crate) gl: GraphicsLibrary,
}

impl Game {
    pub fn new() -> GameBuilder {
        GameBuilder {}
    }
}

pub struct GameBuilder {}

impl GameBuilder {
    pub fn build(self) -> Result<Game, Box<dyn Error>> {
        Ok(Game {
            gl: GraphicsLibrary::new()?,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.gl.start_frame();
    }
}

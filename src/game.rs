use std::error::Error;

use miniquad::{conf::Conf, EventHandler};

use crate::{
    gl::GraphicsContext,
    math::{
        color::Color,
        point::{pt2i, Pt2i},
    },
};

pub struct Game {
    clear_color: Color,
    window_size: Pt2i,

    pub(crate) gl: GraphicsContext,
}

impl Game {
    pub fn new() -> GameBuilder {
        GameBuilder {
            clear_color: Color::BLACK,
            window_title: None,
            window_size: pt2i(800, 600),
        }
    }

    pub fn window_size(&self) -> &Pt2i {
        &self.window_size
    }
}

pub struct GameBuilder {
    clear_color: Color,
    window_size: Pt2i,
    window_title: Option<String>,
}

impl GameBuilder {
    pub fn start(self) -> Result<(), Box<dyn Error>> {
        let game = Game {
            clear_color: self.clear_color,
            window_size: self.window_size,

            gl: GraphicsContext::new()?,
        };

        miniquad::start(
            Conf {
                window_title: self.window_title.unwrap_or_else(String::new),
                window_width: self.window_size.x,
                window_height: self.window_size.y,

                ..Default::default()
            },
            move || Box::new(game),
        );
        Ok(())
    }

    pub fn clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }
    pub fn window_size(mut self, size: impl Into<Pt2i>) -> Self {
        self.window_size = size.into();
        self
    }
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = Some(title.into());
        self
    }
}

impl EventHandler for Game {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.gl.clear(self.clear_color);
    }
}

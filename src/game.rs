use std::sync::Arc;

use assets::{Asset, Assets, GlAsset};
use input::Input;
use miniquad::{conf::Conf, date, window::dpi_scale, EventHandler, KeyCode, MouseButton};

use crate::{
    gl::GraphicsContext,
    math::{
        color::Color,
        point::{pt2, pt2i, Pt2, Pt2i},
    },
    obj::{MakeDefault, Obj},
};

pub mod assets;
pub mod input;

pub struct Game {
    clear_color: Color,
    window_size: Pt2,

    /// unix time in seconds since window launched
    pub(crate) time_start: f64,
    /// unix time in seconds since last update() call
    pub(crate) last_frame: f64,

    pub(crate) scene: Option<Box<dyn Obj>>,
    pub(crate) gl: GraphicsContext,

    pub assets: Assets,
    pub input: Input,
}

pub struct GameBuilder {
    clear_color: Color,
    window_size: Pt2i,
    window_title: Option<String>,
}

impl Game {
    pub fn new() -> GameBuilder {
        GameBuilder {
            clear_color: Color::BLACK,
            window_title: None,
            window_size: pt2i(800, 600),
        }
    }
}

impl GameBuilder {
    pub fn start<Scene>(self)
    where
        Scene: MakeDefault + 'static,
    {
        miniquad::start(
            Conf {
                window_title: self.window_title.unwrap_or_else(String::new),
                window_width: self.window_size.x,
                window_height: self.window_size.y,

                ..Default::default()
            },
            move || {
                let mut game = Game {
                    clear_color: self.clear_color,
                    window_size: self.window_size.into(),

                    time_start: date::now(),
                    last_frame: date::now(),

                    scene: None,
                    gl: GraphicsContext::new().unwrap(),

                    assets: Assets::new(),
                    input: Input::new(),
                };
                game.scene = Some(Box::new(Scene::make_default(&mut game)));

                Box::new(game)
            },
        );
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

impl Game {
    pub fn window_size(&self) -> &Pt2 {
        &self.window_size
    }
    pub fn time(&self) -> f32 {
        (self.time_start - date::now()) as f32
    }

    /// Load asset to GPU. This is used for textures and shaders.
    #[expect(private_bounds)]
    pub fn load_gl_asset<T: GlAsset>(&mut self, path: &'static str) -> Arc<T> {
        self.assets.load_gl(path, &mut self.gl)
    }
    /// Load asset. Equivelent to game.assets.load
    pub fn load_asset<T: Asset>(&mut self, path: &'static str) -> Arc<T> {
        self.assets.load(path)
    }
}

impl EventHandler for Game {
    fn update(&mut self) {
        if let Some(mut scene) = self.scene.take() {
            let now = date::now();
            let delta = now - self.last_frame;

            scene.update_children(self, delta as f32);

            self.scene = Some(scene);
            self.last_frame = now;
        } else {
            println!("failed to get scene")
        }
    }

    fn draw(&mut self) {
        self.gl.start_frame(self.clear_color);
        if let Some(scene) = self.scene.as_ref() {
            scene.draw_children(&mut self.gl)
        }
        self.gl.finish();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.window_size = pt2(width, height);
        self.gl.update_viewport_transform(self.window_size);
    }

    fn key_down_event(&mut self, key: miniquad::KeyCode, _: miniquad::KeyMods, repeat: bool) {
        if !repeat {
            self.input.set_key_down(key)
        }
    }
}

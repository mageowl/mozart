use crate::{
    game::Game,
    gl::GraphicsContext,
    math::{transform::Transform, Seconds},
};

pub use mozart_macro::{Obj, Obj2d};

#[doc(hidden)]
pub mod maybe;

pub mod sprite;

pub trait Obj {
    fn update_children(&mut self, game: &mut Game, delta: Seconds);
    fn draw_children(&self, ctx: &mut GraphicsContext);
}

pub trait Obj2d: Obj {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
}

pub trait Make: Obj {
    type Config;

    fn make(game: &mut Game, config: Self::Config) -> Self;
}

pub trait MakeDefault: Make {
    fn make_default(game: &mut Game) -> Self;
}
impl<T, C> MakeDefault for T
where
    T: Make<Config = C>,
    C: Default,
{
    fn make_default(game: &mut Game) -> Self {
        Self::make(game, Default::default())
    }
}

pub trait Update: Obj {
    fn update(&mut self, game: &mut Game, delta: Seconds);
}

pub trait Draw: Obj {
    fn draw(&self, ctx: &mut GraphicsContext);
}

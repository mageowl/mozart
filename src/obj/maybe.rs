use crate::{game::Game, gl::GraphicsContext};

use super::{Draw, Obj, Update};

#[repr(transparent)]
pub struct Wrapper<T>(pub T);

pub trait MaybeUpdate {
    fn maybe_update(&mut self, game: &mut Game, delta: f32);
}
impl<T: Update> MaybeUpdate for &mut Wrapper<&mut T> {
    #[inline(always)]
    fn maybe_update(&mut self, game: &mut Game, delta: f32) {
        self.0.update(game, delta)
    }
}

impl<T> MaybeUpdate for &mut &mut Wrapper<&mut T> {
    fn maybe_update(&mut self, _: &mut Game, _: f32) {}
}

#[macro_export]
macro_rules! maybe_update {
    ($obj:expr, $game:expr, $delta:expr) => {{
        use $crate::obj::maybe::MaybeUpdate;
        let obj = $obj;
        (&mut &mut $crate::obj::maybe::Wrapper(obj)).maybe_update($game, $delta)
    }};
}

pub trait MaybeDraw {
    fn maybe_draw(&self, ctx: &mut GraphicsContext);
}
impl<T: Draw> MaybeDraw for &Wrapper<&T> {
    #[inline(always)]
    fn maybe_draw(&self, ctx: &mut GraphicsContext) {
        self.0.draw(ctx)
    }
}

impl<T> MaybeDraw for &&Wrapper<&T> {
    fn maybe_draw(&self, _: &mut GraphicsContext) {}
}

#[macro_export]
macro_rules! maybe_draw {
    ($obj:expr, $ctx:expr) => {{
        use $crate::obj::maybe::MaybeDraw;
        (&&$crate::obj::maybe::Wrapper($obj)).maybe_draw($ctx)
    }};
}

pub trait MaybeUpdateChildren {
    fn maybe_update_children(self, game: &mut Game, delta: f32);
}
impl<T: Obj> MaybeUpdateChildren for &mut Wrapper<&mut T> {
    #[inline(always)]
    fn maybe_update_children(self, game: &mut Game, delta: f32) {
        self.0.update_children(game, delta)
    }
}

impl<T> MaybeUpdateChildren for &mut &mut Wrapper<&mut T> {
    #[inline(always)]
    fn maybe_update_children(self, _: &mut Game, _: f32) {}
}

#[macro_export]
macro_rules! maybe_update_children {
    ($obj:expr, $game:expr, $delta:expr) => {};
}

pub trait MaybeDrawChildren {
    fn maybe_draw_children(self, ctx: &mut GraphicsContext);
}
impl<T: Obj> MaybeDrawChildren for &&Wrapper<&T> {
    #[inline(always)]
    fn maybe_draw_children(self, ctx: &mut GraphicsContext) {
        self.0.draw_children(ctx)
    }
}

impl<T> MaybeDrawChildren for &Wrapper<&T> {
    #[inline(always)]
    fn maybe_draw_children(self, _: &mut GraphicsContext) {}
}

#[macro_export]
macro_rules! maybe_draw_children {
    ($obj:expr, $ctx:expr) => {{
        use $crate::obj::maybe::MaybeDrawChildren;
        (&&$crate::obj::maybe::Wrapper($obj)).maybe_draw_children($ctx)
    }};
}

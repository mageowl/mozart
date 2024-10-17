use mozart::{
    game::Game,
    obj::{sprite::Sprite, Make, Obj},
};

struct Scene {
    sprite: Sprite,
}

impl mozart::obj::Obj for Scene {
    fn draw_children(&self, ctx: &mut mozart::gl::GraphicsContext) {
        {
            use ::mozart::obj::maybe::MaybeDraw;
            (&&::mozart::obj::maybe::Wrapper(self)).maybe_draw(ctx)
        };
        {
            use ::mozart::obj::maybe::MaybeDrawChildren;
            (&&::mozart::obj::maybe::Wrapper(&self.sprite)).maybe_draw_children(ctx)
        };
    }
    fn update_children(&mut self, game: &mut mozart::game::Game, delta: f32) {
        use mozart::obj::maybe::MaybeUpdateChildren;
        (&mut &mut mozart::obj::maybe::Wrapper(self)).maybe_update_children(game, delta);
    }
}

impl Make for Scene {
    type Config = ();

    fn make(game: &mut Game, _: Self::Config) -> Self {
        Self {
            sprite: Sprite::make(game, Sprite::cfg_from_texture("examples/assets/sprite.png")),
        }
    }
}

fn main() {
    Game::new().start::<Scene>()
}

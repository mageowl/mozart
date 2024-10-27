use mozart::{
    game::Game,
    math::transform::Transform,
    obj::{sprite::Sprite, Make, Obj},
};

#[derive(Obj)]
struct Scene {
    sprite: Sprite,
}

impl Make for Scene {
    type Config = ();

    fn make(game: &mut Game, _: Self::Config) -> Self {
        Self {
            sprite: Sprite::make(
                game,
                Sprite::cfg_from_texture("examples/assets/sprite.png")
                    .transform(Transform::IDENTITY.scaled_uniform(16.)),
            ),
        }
    }
}

fn main() {
    Game::new().start::<Scene>()
}

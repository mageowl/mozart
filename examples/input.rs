use mozart::{
    game::Game,
    math::transform::Transform,
    obj::{sprite::Sprite, Make, Obj, Obj2d, Update},
};

#[derive(Obj, Obj2d)]
struct Player {
    transform: Transform,
    sprite: Sprite,
}

impl Make for Player {
    type Config = Transform;

    fn make(game: &mut Game, transform: Self::Config) -> Self {
        Self {
            transform,
            sprite: Sprite::make(
                game,
                Sprite::cfg_from_texture("./examples/assets/sprite.png")
                    .transform(Transform::IDENTITY.scaled_uniform(4.).with_pivot((8., 8.))),
            ),
        }
    }
}

#[derive(Obj)]
struct Scene {
    player: Player,
}

impl Make for Scene {
    type Config = ();

    fn make(game: &mut Game, _: Self::Config) -> Self {
        dbg!(game.window_size());
        Self {
            player: Player::make(
                game,
                Transform::IDENTITY.with_offset(*game.window_size() / 2.),
            ),
        }
    }
}

fn main() {
    Game::new().start::<Scene>();
}

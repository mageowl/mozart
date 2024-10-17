#[derive(Obj, Obj2d)]
struct Player {
    transform: Transform,
    sprite: Sprite,
    collider: Collider,

    velocity: Pt2,
}

impl Player {
    const SPRITE: AssetId<Texture> = asset!("player.png");
    const SPEED: f64 = 200.;
    const ACCELERATION: f64 = 0.9;
}

impl Make for Player {
    type Config = ();

    fn make(game: &mut Game, _cfg: ()) -> Self {
        Self {
            transform: Transform::IDENTITY,
            sprite: Sprite::make(game, Sprite::cfg_from_texture(Self::SPRITE)),
            collider: Collider::make(
                game,
                Collider::cfg_from_shape(
                    Rect {
                        size: pt2(8., 8.),
                        offset: pt2(-8., -8.),
                    },
                    "player",
                ),
            ),

            velocity: Pt2::ZERO,
        }
    }
}

impl Update for Player {
    fn update(&mut self, game: &mut Game, delta: f64) {
        let input = game.get_server::<InputServer>();

        let direction = input.get_axis("left", "right");
        if direction != 0 {
            math::move_toward(
                &mut self.velocity.x,
                direction * Self::SPEED,
                Self::ACCELERATION * Self::SPEED,
            );
        } else {
            math::move_toward(&mut self.velocity.x, 0, Self::FRICTION * Self::SPEED);
        }

        self.collider
            .move_and_slide(&mut self.transform, self.velocity);
    }
}

#[derive(Obj)]
struct TestScene {
    player: Player,
    map: TileMap,
}

const TEST_MAP: Arc<LDTKMap> = asset!("test_map.ldtk");

impl Make for TestScene {
    type Config = ();

    fn make(game: &mut Game, _cfg: ()) -> Self {
        Self {
            player: Player::make(game),
            map: TileMap::make(
                game,
                TileMap::cfg_from_map(TEST_MAP).collision_layer("level"),
            ),
        }
    }
}

fn main() {
    Game::new()
        .add_server(CollisionServer::new(vec!["player", "level"]))
        .start::<TestScene>();
}

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
        let transform = Transform::new();

        Self {
            transform,
            sprite: Sprite::make(
                game,
                Sprite::cfg_from_texture(Self::SPRITE).parent(transform),
            ),
            collider: Collider::make(
                game,
                Collider::cfg_from_shape(
                    Rect {
                        size: pt2(8., 8.),
                        offset: pt2(-8., -8.),
                    },
                    "player",
                )
                .parent(transform),
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
    }
}

#[derive(Obj)]
struct TestScene {
    player: Player,
    map: TileMap,
}

const TEST_MAP: AssetId<LDTKMap> = asset!("test_map.ldtk");

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
    let game = Game::new();
    game.add_server(CollisionServer::new(vec!["player", "level"]));

    let scene = TestScene::make(&mut game);
    game.event_loop(scene);
}

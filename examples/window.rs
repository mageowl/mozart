use mozart::{
    game::Game,
    gl::GraphicsContext,
    obj::{Draw, Make, Obj},
};

#[derive(Obj)]
struct TestScene;

impl Make for TestScene {
    type Config = ();

    fn make(_game: &mut Game, _config: Self::Config) -> Self {
        Self
    }
}

impl Draw for TestScene {
    fn draw(&self, _ctx: &mut GraphicsContext) {}
}

fn main() {
    Game::new().start::<TestScene>()
}

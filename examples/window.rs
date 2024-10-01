use std::error::Error;

use mozart::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    Game::new().start()
}

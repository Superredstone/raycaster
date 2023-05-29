use std::collections::HashMap;

use game::Game;
use macroquad::prelude::*;

mod game;

#[macroquad::main(window_conf)]
async fn main() {
    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    textures.insert(
        String::from("sample_wall"),
        load_texture("textures/sample_wall.png").await.unwrap(),
    );

    let mut game = Game::new(textures);

    game.change_screen_size(game.window_size.x, game.window_size.y);

    loop {
        // Update
        game.update();

        // Draw
        game.draw();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My awesome game".to_owned(),
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

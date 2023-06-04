use std::collections::HashMap;

use game::Game;
use macroquad::prelude::*;

mod game;

#[macroquad::main(window_conf)]
async fn main() {
    let (textures, textures_datas, textures_image_datas) = init_textures().await;

    let mut game = Game::new(textures, textures_datas, textures_image_datas);

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

async fn init_textures() -> (
    HashMap<String, Texture2D>,
    HashMap<String, Image>,
    HashMap<String, Vec<[u8; 4]>>,
) {
    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    let mut textures_datas: HashMap<String, Image> = HashMap::new();
    let mut textures_image_datas: HashMap<String, Vec<[u8; 4]>> = HashMap::new();

    insert_texture(
        &mut textures,
        &mut textures_datas,
        &mut textures_image_datas,
        "textures/sample_wall.png",
        "sample_wall".to_string(),
    )
    .await;

    let img = Image {
        bytes: [0, 1, 10, 3].to_vec(),
        width: 1,
        height: 1,
    };

    textures.insert(String::from("test"), Texture2D::from_image(&img));

    (textures, textures_datas, textures_image_datas)
}

async fn insert_texture(
    textures: &mut HashMap<String, Texture2D>,
    textures_datas: &mut HashMap<String, Image>,
    textures_image_datas: &mut HashMap<String, Vec<[u8; 4]>>,
    path: &str,
    name: String,
) {
    let new_texture = load_texture(path).await.unwrap();
    let texture_data = new_texture.get_texture_data();

    textures.insert(name.clone(), new_texture);
    textures_datas.insert(name.clone(), texture_data.clone());

    // Devo prendere lo slice di 4 byte che ritorna la texture e renderlo un vettore
    textures_image_datas.insert(name, texture_data.get_image_data().to_vec());
}

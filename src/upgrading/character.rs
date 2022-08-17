use imagesize::size;

use macroquad::prelude::*;

#[derive(Clone)]
pub struct Character {
    pub character: String,
    pub can_set: bool,
    pub texture: Texture2D,
    pub texture_size: (f32, f32),
    pub texture_file: String,
}

impl Character {
    pub async fn new() -> Self {
        Self {
            texture: load_texture("res/display/apple.png").await.unwrap(),
            can_set: true,
            character: "Apple".to_string(),
            texture_size: (
                size("res/regular/apple.png").unwrap().width as f32,
                size("res/regular/apple.png").unwrap().height as f32,
            ),
            texture_file: "res/display/apple.png".to_string(),
        }
    }

    pub async fn set(&mut self, texture: String) {
        self.texture = load_texture(
            ("res/display/".to_string() + texture.to_lowercase().as_str() + ".png").as_str(),
        )
        .await
        .unwrap();

        self.character = texture.to_string();

        self.texture_file = "res/display/".to_string() + texture.to_lowercase().as_str() + ".png"
    }

    pub fn draw(&mut self) {
        // Draw the Character
        draw_texture(
            self.texture,
            screen_width() / 2. - size(self.texture_file.clone()).unwrap().width as f32 / 2.,
            screen_height() / 2. - size(self.texture_file.clone()).unwrap().height as f32 / 2.,
            WHITE,
        )
    }
}

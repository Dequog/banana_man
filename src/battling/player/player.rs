use crate::battling::enemy::Enemy;

use imagesize::size;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

// Player size is equal to the size of the player image
const PLAYER_SIZE: Lazy<Vec2> = Lazy::new(|| {
    const_vec2!([
        size("res/regular/apple.png").unwrap().width as f32,
        size("res/regular/apple.png").unwrap().height as f32,
    ])
});

pub const PLAYER_SPEED: f32 = 250.;

pub struct Player {
    pub rect: Rect,
    pub texture: String,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            // A rect will represent the player bounds
            rect: Rect::new(
                screen_width() / 2. - PLAYER_SIZE[0] / 2.,
                screen_height() / 2. - PLAYER_SIZE[1] / 2.,
                PLAYER_SIZE[0],
                PLAYER_SIZE[1],
            ),
            texture: "res/regular/apple.png".to_string(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        match (
            is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) || is_key_down(KeyCode::H),
            is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) || is_key_down(KeyCode::L),
        ) {
            // Move the player to the left
            (true, _) => {
                self.rect.x -= dt * PLAYER_SPEED;
            }
            // Move the player to the right
            (_, true) => {
                self.rect.x += dt * PLAYER_SPEED;
            }
            _ => {}
        }

        match (
            is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::K),
            is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) || is_key_down(KeyCode::J),
        ) {
            // Move the player up
            (true, _) => {
                self.rect.y -= dt * PLAYER_SPEED;
            }
            // Move the player down
            (_, true) => {
                self.rect.y += dt * PLAYER_SPEED;
            }
            _ => {}
        }
    }

    pub async fn draw(&self) {
        // Draw the player
        draw_texture(
            load_texture(self.texture.as_str()).await.unwrap(),
            self.rect.x,
            self.rect.y,
            WHITE,
        );
    }

    pub fn reset(&mut self) {
        // Reset the rect position
        self.rect.x = screen_width() / 2. - PLAYER_SIZE[0] / 2.;
        self.rect.y = screen_height() / 2. - PLAYER_SIZE[1] / 2.
    }

    pub fn is_collision(&self, enemy: Enemy) -> bool {
        // If enemy collides with bullet
        if let Some(_intersection) = enemy.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}

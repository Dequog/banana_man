use imagesize::size;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

// Enemy size is equal to the size of the enemy image
const ENEMY_SIZE: Lazy<Vec2> = Lazy::new(|| {
    const_vec2!([
        size("res/enemy/orange.png").unwrap().width as f32,
        size("res/enemy/orange.png").unwrap().height as f32,
    ])
});
const ENEMY_SPEED: f32 = 250.;

#[derive(PartialEq, Clone)]
pub struct Enemy {
    pub rect: Rect,
    pub is_alive: bool,
    pub has_given_money: bool,
    pub texture: Texture2D,
    vel: Vec2,
    pub worth: i32,
}

impl Enemy {
    pub async fn new(pos: Vec2, player_pos: Vec2) -> Self {
        Self {
            // A rect will represent the enemy bounds
            rect: Rect::new(pos.x, pos.y, ENEMY_SIZE[0], ENEMY_SIZE[0]),
            texture: load_texture("res/enemy/orange.png").await.unwrap(),
            is_alive: true,
            has_given_money: false,
            vel: || -> Vec2 {
                let dir_x = player_pos.clone().x - pos.x;
                let dir_y = player_pos.clone().y - pos.y;

                let factor = ENEMY_SPEED / (dir_x.powi(2) + dir_y.powi(2)).sqrt();

                let vel_x = dir_x * factor;
                let vel_y = dir_y * factor;

                // Return the velocity vector
                vec2(vel_x, vel_y)
            }(),
            worth: 1,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_alive {
            // Update the rect position
            self.rect.x += dt * self.vel.x;
            self.rect.y += dt * self.vel.y;
        }
    }

    pub fn draw(&self) {
        if self.is_alive {
            // Draw the enemy
            draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
        }
    }
}

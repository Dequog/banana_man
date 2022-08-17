use crate::battling::enemy::Enemy;

use imagesize::size;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

// Bullet size is equal to the size of the bullet image
const BULLET_SIZE: Lazy<Vec2> = Lazy::new(|| {
    const_vec2!([
        size("res/bullet.png").unwrap().width as f32,
        size("res/bullet.png").unwrap().height as f32,
    ])
});

const BULLET_SPEED: f32 = 500.;

#[derive(PartialEq, Clone, Copy)]
pub struct Bullet {
    pub rect: Rect,
    texture: Texture2D,
    vel: Vec2,
}

impl Bullet {
    pub async fn new(pos: Vec2) -> Self {
        Self {
            // A rect will represent the bullet bounds
            rect: Rect::new(pos.x, pos.y, BULLET_SIZE[0], BULLET_SIZE[1]),
            texture: load_texture("res/bullet.png").await.unwrap(),
            vel: || -> Vec2 {
                let dir_x = mouse_position().clone().0 - pos.x;
                let dir_y = mouse_position().clone().1 - pos.y;

                let factor = BULLET_SPEED / (dir_x.powi(2) + dir_y.powi(2)).sqrt();

                let vel_x = dir_x * factor;
                let vel_y = dir_y * factor;

                // Return the velocity vector
                vec2(vel_x, vel_y)
            }(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update the rect position
        self.rect.x += dt * self.vel.x;
        self.rect.y += dt * self.vel.y;
    }

    pub fn draw(&self) {
        // Draw the bullet
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }

    pub fn is_collision(&self, enemy: Enemy) -> bool {
        // If enemy collides with bullet
        if let Some(_intersection) = enemy.rect.intersect(self.rect) {
            return true;
        }

        false
    }
}

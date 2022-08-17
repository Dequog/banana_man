use crate::objects::button::Button;

use macroquad::prelude::*;

pub struct BulletButton {
    button: Button,
    number: f32,
    bullet_count: f32,
    can_press: bool,

    pub bullets: usize,
    pub money: usize,
}

impl BulletButton {
    pub async fn new(number: f32, bullet_count: f32) -> Self {
        Self {
            button: Button::new(
                vec2(screen_width() - 220., screen_height() - 60.),
                vec2(200., 50.),
                format!("{}x Bullet(s)", bullet_count).to_string(),
                GRAY,
                15,
            )
            .await,
            number,
            bullet_count,
            bullets: 0,
            money: 0,
            can_press: true,
        }
    }

    pub fn update(&mut self) {
        self.button.update(
            vec2(screen_width() - 200., screen_height() - self.number * 60.),
            self.button.rect.size(),
        );

        if self.button.is_pressed() && self.can_press && self.money >= self.bullet_count as usize {
            self.bullets += self.bullet_count as usize;
            self.money -= self.bullet_count as usize;

            self.can_press = false;
        } else if !self.button.is_pressed() {
            self.can_press = true;
        }
    }

    pub async fn draw(&mut self) {
        self.button.draw().await;
    }
}

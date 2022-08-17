use crate::objects::text::Text;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Bullets {
    text: Text,
    pub bullets: usize,
}

impl Bullets {
    pub async fn new() -> Self {
        Self {
            bullets: 3,
            text: Text::new(
                vec2(20., 40.),
                "res/Roboto-Medium.ttf".to_string(),
                format!("Bullets: {}", 3.to_string()),
                30,
                BLACK,
            )
            .await,
        }
    }

    pub fn reset(&mut self) {
        self.bullets = 3;
    }

    pub fn decrement(&mut self) {
        self.text.pos = vec2(20., 40.);

        // Decrement the bullets
        self.bullets -= 1;
    }

    pub fn draw(&mut self) {
        self.text
            .change(format!("Bullets: {}", self.bullets.to_string()));

        // Draw the Bullets
        self.text.draw();
    }
}

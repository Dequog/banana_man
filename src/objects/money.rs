use crate::objects::text::Text;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Money {
    text: Text,
    pub money: usize,
}

impl Money {
    pub async fn new() -> Self {
        Self {
            money: 0,
            text: Text::new(
                vec2(
                    screen_width() * 0.5 - 7.5 - "$0".chars().count() as f32 * 7.5,
                    40.,
                ),
                "res/Roboto-Medium.ttf".to_string(),
                format!("${}", 0.to_string()),
                30,
                BLACK,
            )
            .await,
        }
    }

    pub fn increment(&mut self, amount: usize) {
        self.text.pos = vec2(
            screen_width() * 0.5
                - 7.5
                - format!("${}", self.money.to_string()).chars().count() as f32 * 7.5,
            40.,
        );

        // Increment the money
        self.money += amount;
    }

    pub fn reset(&mut self) {
        self.money = 0;
    }

    pub fn draw(&mut self) {
        self.text.change(format!("${}", self.money.to_string()));

        self.text.pos = vec2(
            screen_width() * 0.5
                - 7.5
                - format!("${}", self.money.to_string()).chars().count() as f32 * 7.5,
            40.,
        );

        // Draw the Money
        self.text.draw();
    }
}

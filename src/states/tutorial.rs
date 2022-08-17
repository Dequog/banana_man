use crate::objects::button::Button;
use crate::states::game_state::GameState;

use imagesize::size;

use macroquad::prelude::*;

pub struct Tutorial {
    pub game_state: GameState,

    messages: [[String; 2]; 8],
    message_index: usize,

    options: Vec<Button>,

    can_press_next_button: bool,
    can_press_previous_button: bool,
}

impl Tutorial {
    pub async fn new() -> Self {
        return Self {
            game_state: GameState::Tutorial,

            messages: [
                [
                    "Use WASD keys or arrow keys to move".to_string(),
                    "".to_string(),
                ],
                [
                    "Click to shoot".to_string(),
                    "res/tutorial/shoot.png".to_string(),
                ],
                [
                    "When you kill an enemy, you get money".to_string(),
                    "res/tutorial/kill.png".to_string(),
                ],
                [
                    "If you get hit or run out of bullets, you die".to_string(),
                    "res/tutorial/die.png".to_string(),
                ],
                [
                    "Click retreat button to go back to the upgrades".to_string(),
                    "res/tutorial/retreat.png".to_string(),
                ],
                [
                    "Click the upgrade button to upgrade your character".to_string(),
                    "res/tutorial/upgrade.png".to_string(),
                ],
                [
                    "Click the buy bullets button to buy bullets ($1)".to_string(),
                    "res/tutorial/buy_bullets.png".to_string(),
                ],
                [
                    "Click the battle button to go back to battling".to_string(),
                    "res/tutorial/battle.png".to_string(),
                ],
            ],
            message_index: 0,

            options: vec![
                Button::new(
                    vec2(screen_width() - 220., screen_height() - 100.),
                    vec2(200., 50.),
                    "Next".to_string(),
                    RED,
                    15,
                )
                .await,
                Button::new(
                    vec2(20., screen_height() - 100.),
                    vec2(200., 50.),
                    "Previous".to_string(),
                    RED,
                    15,
                )
                .await,
                Button::new(
                    vec2(20., 20.),
                    vec2(200., 50.),
                    "Close".to_string(),
                    RED,
                    15,
                )
                .await,
            ],

            can_press_next_button: false,
            can_press_previous_button: false,
        };
    }

    pub async fn start(&mut self) {
        // Draw message
        draw_text_ex(
            self.messages[self.message_index as usize][0].as_str(),
            screen_width() * 0.5
                - 7.
                - self.messages[self.message_index as usize][0]
                    .to_string()
                    .chars()
                    .count() as f32
                    * 7.,
            100.,
            TextParams {
                font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                font_size: 30,
                color: BLACK,
                font_scale: 1.,
                font_scale_aspect: 1.,
            },
        );

        if self.messages[self.message_index as usize][1] != "" {
            // Draw texture
            draw_texture(
                load_texture(self.messages[self.message_index as usize][1].as_str())
                    .await
                    .unwrap(),
                screen_width() * 0.5 - 200.,
                screen_height() / 2.
                    - (size(self.messages[self.message_index as usize][1].clone())
                        .unwrap()
                        .height
                        / 2) as f32,
                WHITE,
            );
        }

        // Draw buttons
        for option in &mut self.options {
            // Update button position
            match option.text.as_str() {
                "Next" => {
                    if self.message_index < self.messages.len() - 1 {
                        option.draw().await;

                        option.update(
                            vec2(screen_width() - 220., screen_height() - 100.),
                            vec2(200., 50.),
                        );

                        if option.is_pressed() && self.can_press_next_button {
                            self.message_index += 1;

                            self.can_press_next_button = false;
                        } else if !option.is_pressed() {
                            self.can_press_next_button = true;
                        }
                    }
                }
                "Previous" => {
                    if self.message_index > 0 {
                        option.draw().await;

                        option.update(vec2(20., screen_height() - 100.), vec2(200., 50.));

                        if option.is_pressed() && self.can_press_previous_button {
                            self.message_index -= 1;

                            self.can_press_previous_button = false;
                        } else if !option.is_pressed() {
                            self.can_press_previous_button = true;
                        }
                    }
                }
                "Close" => {
                    option.draw().await;

                    option.update(vec2(20., 20.), vec2(200., 50.));

                    if option.is_pressed() {
                        self.game_state = GameState::Menu;
                    }
                }

                _ => {}
            }
        }
    }
}

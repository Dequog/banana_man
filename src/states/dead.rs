use crate::objects::button::Button;
use crate::states::game_state::GameState;

use macroquad::prelude::*;

pub struct Dead {
    pub game_state: GameState,
    pub out_of_bullets: bool,

    title: String,
    options: Vec<Button>,

    can_press_options: bool,
}

impl Dead {
    pub async fn new() -> Self {
        return Self {
            game_state: GameState::Dead,
            out_of_bullets: false,

            title: "You Died".to_string(),
            options: vec![
                Button::new(
                    vec2(screen_width() / 2. - 100., screen_height() / 2. - 200.),
                    vec2(200., 50.),
                    "Menu".to_string(),
                    RED,
                    15,
                )
                .await,
                Button::new(
                    vec2(screen_width() - 220., 20.),
                    vec2(200., 50.),
                    "Try Again".to_string(),
                    RED,
                    14,
                )
                .await,
            ],

            can_press_options: false,
        };
    }

    pub async fn start(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            self.can_press_options = true;
        }

        // Draw title
        draw_text_ex(
            if self.out_of_bullets {
                self.title = "You ran out of bullets".to_string();

                self.title.as_str()
            } else {
                self.title = "You died".to_string();

                self.title.as_str()
            },
            screen_width() * 0.5 - 7. - self.title.to_string().chars().count() as f32 * 7.,
            30.,
            TextParams {
                font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
                font_size: 30,
                color: BLACK,
                font_scale: 1.,
                font_scale_aspect: 1.,
            },
        );

        // Draw buttons
        for option in &mut self.options {
            option.draw().await;

            // Update button position
            match option.text.as_str() {
                "Try Again" => {
                    option.update(
                        vec2(screen_width() / 2. - 100., screen_height() / 2. - 55.),
                        vec2(200., 50.),
                    );

                    if option.is_pressed() && self.can_press_options {
                        self.game_state = GameState::Battling;
                        self.can_press_options = false;
                    }
                }
                "Menu" => {
                    option.update(
                        vec2(screen_width() / 2. - 100., screen_height() / 2. + 5.),
                        vec2(200., 50.),
                    );

                    if option.is_pressed() && self.can_press_options {
                        self.game_state = GameState::Menu;
                        self.can_press_options = false;
                    }
                }
                _ => {}
            }
        }
    }
}

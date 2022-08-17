use crate::objects::bullets::Bullets;
use crate::objects::button::Button;
use crate::objects::money::Money;
use crate::states::game_state::GameState;
use crate::upgrading::bullet_button::BulletButton;
use crate::upgrading::character::Character;

use macroquad::prelude::*;

pub struct Upgrading {
    pub game_state: GameState,
    pub money: Money,
    pub bullets: Bullets,
    pub character: Character,

    // (Character Name, Price)
    characters: [(String, usize); 4],

    upgrade_button: Button,
    buy_bullets_button: Button,
    battle_button: Button,

    buy_bullet_buttons: [BulletButton; 4],

    can_press_buy_bullet_buttons: bool,
    is_showing_buy_bullet_buttons: bool,
}

impl Upgrading {
    pub async fn new() -> Self {
        return Self {
            game_state: GameState::Upgrading,
            money: Money::new().await,
            bullets: Bullets::new().await,
            character: Character::new().await,

            characters: [
                ("Orange".to_string(), 100),
                ("Pear".to_string(), 500),
                ("Pineapple".to_string(), 1000),
                ("Banana".to_string(), 10000),
            ],

            buy_bullet_buttons: [
                BulletButton::new(1., 100.).await,
                BulletButton::new(2., 50.).await,
                BulletButton::new(3., 10.).await,
                BulletButton::new(4., 1.).await,
            ],
            upgrade_button: Button::new(
                vec2(20., screen_height() - 180.),
                vec2(200., 50.),
                "Upgrade".to_string(),
                GRAY,
                15,
            )
            .await,
            buy_bullets_button: Button::new(
                vec2(20., screen_height() - 120.),
                vec2(200., 50.),
                "Buy Bullets".to_string(),
                GRAY,
                15,
            )
            .await,
            battle_button: Button::new(
                vec2(20., screen_height() - 60.),
                vec2(200., 50.),
                "Battle".to_string(),
                GRAY,
                15,
            )
            .await,

            can_press_buy_bullet_buttons: true,
            is_showing_buy_bullet_buttons: false,
        };
    }

    pub async fn start(&mut self) {
        self.battle_button.draw().await;
        self.battle_button
            .update(vec2(20., screen_height() - 60.), vec2(200., 50.));

        self.upgrade_button.draw().await;
        self.upgrade_button.update(
            vec2(self.buy_bullets_button.rect.x, screen_height() - 180.),
            self.upgrade_button.rect.size(),
        );

        self.buy_bullets_button.draw().await;
        self.buy_bullets_button.update(
            vec2(self.buy_bullets_button.rect.x, screen_height() - 120.),
            self.buy_bullets_button.rect.size(),
        );

        self.money.draw();
        self.bullets.draw();
        self.character.draw();

        // Check if you need to upgrade the character
        self.check_upgrade().await;

        // Check if you need to battle
        self.check_battle_button();

        // Change the upgrade button text
        self.set_upgrade_button_text();

        // Everything to do with the bullet buttons
        self.buy_bullet_buttons_stuff().await;
    }

    fn set_upgrade_button_text(&mut self) {
        self.upgrade_button.text = format!(
            "Upgrade: {}",
            if self.character.character == "Banana".to_string() {
                "Max".to_string()
            } else {
                format!(
                    "${}",
                    // Cost to upgrade (The price of the next character)
                    self.characters[self
                        .characters
                        .iter()
                        .position(|r| *r.0 == self.character.character)
                        .unwrap_or_default()]
                    .1
                    .to_string()
                )
            }
        )
        .to_string();
    }

    fn check_battle_button(&mut self) {
        if self.battle_button.is_pressed() {
            self.game_state = GameState::Battling;
        }
    }

    async fn buy_bullet_buttons_stuff(&mut self) {
        if !self.buy_bullets_button.is_pressed() {
            self.can_press_buy_bullet_buttons = true
        }

        if self.buy_bullets_button.is_pressed()
            && !self.is_showing_buy_bullet_buttons
            && self.can_press_buy_bullet_buttons
        {
            self.is_showing_buy_bullet_buttons = true;
            self.can_press_buy_bullet_buttons = false;
        } else if self.buy_bullets_button.is_pressed()
            && self.is_showing_buy_bullet_buttons
            && self.can_press_buy_bullet_buttons
        {
            self.is_showing_buy_bullet_buttons = false;
            self.can_press_buy_bullet_buttons = false;
        }

        if self.is_showing_buy_bullet_buttons {
            for button in self.buy_bullet_buttons.iter_mut() {
                // Give button some data
                button.bullets = self.bullets.bullets;
                button.money = self.money.money;

                button.draw().await;
                button.update();

                // Take the mutated data from the button
                self.bullets.bullets = button.bullets;
                self.money.money = button.money;
            }
        }
    }

    async fn check_upgrade(&mut self) {
        if self.upgrade_button.is_pressed()
            // Check if there are new characters to purchase
            && self
                .characters
                .iter()
                .position(|r| *r.0 == self.character.character)
                .unwrap_or_default()
                < self.characters.len() - 1
            && self.character.can_set
            // Check if you have enough money to purchase the new character
            && self.money.money
                >= self.characters[self
                    .characters
                    .iter()
                    .position(|r| *r.0 == self.character.character)
                    .unwrap_or_default()]
                .1
        {
            // Take away money
            self.money.money -= self.characters[self
                .characters
                .iter()
                .position(|r| *r.0 == self.character.character)
                .unwrap_or_default()]
            .1;

            // Change character
            self.character
                .set(if self.character.character == "Apple" {
                    "Orange".to_string()
                } else {
                    (&self.characters[self
                        .characters
                        .iter()
                        .position(|r| *r.0 == self.character.character)
                        .unwrap_or_default()
                        + 1]
                    .0)
                        .to_owned()
                })
                .await;

            self.character.can_set = false;
        } else if !self.upgrade_button.is_pressed() {
            self.character.can_set = true;
        }
    }
}

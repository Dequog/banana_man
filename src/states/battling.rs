use crate::battling::bullet::Bullet;
use crate::battling::enemy::Enemy;
use crate::battling::player::gun::Gun;
use crate::battling::player::player::Player;
use crate::objects::bullets::Bullets;
use crate::objects::button::Button;
use crate::objects::money::Money;
use crate::states::game_state::GameState;
use crate::upgrading::character::Character;

use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;

const ENEMY_SPAWN_TIME: f32 = 50.;

pub struct Battling {
    pub game_state: GameState,

    pub money: Money,
    pub bullet_count: Bullets,
    pub character: Character,
    pub gun: Gun,

    pub out_of_bullets: bool,

    player: Player,
    retreat_button: Button,

    // (Character Name, Price)
    characters: [(String, usize); 4],

    // There can be multiple bullets/enemies in the game, so to keep track of them, they go into a vector
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,

    enemy_spawn_timer: f32,
}

impl Battling {
    pub async fn new() -> Self {
        return Self {
            game_state: GameState::Battling,

            money: Money::new().await,
            bullet_count: Bullets::new().await,
            character: Character::new().await,
            gun: Gun::new().await,

            out_of_bullets: false,

            player: Player::new().await,
            retreat_button: Button::new(
                vec2(screen_width() - 220., 20.),
                vec2(200., 50.),
                "Retreat".to_string(),
                RED,
                15,
            )
            .await,

            characters: [
                ("Orange".to_string(), 100),
                ("Pear".to_string(), 500),
                ("Pineapple".to_string(), 1000),
                ("Banana".to_string(), 10000),
            ],

            bullets: Vec::new(),
            enemies: Vec::new(),

            enemy_spawn_timer: ENEMY_SPAWN_TIME,
        };
    }

    pub async fn start(&mut self) {
        self.gun.bullets = self.bullet_count.bullets;

        self.player.draw().await;
        self.player.update(get_frame_time());

        self.gun.draw();
        self.gun
            .update(self.player.rect.point() + self.player.rect.size() * 0.5);

        // Check if you need to reset the field/game
        self.check_reset().await;

        // Check if you need to shoot a new bullet
        self.check_shoot_bullet().await;

        // Update/Draw all the bullets
        self.update_bullets();

        // Check if you need to spawn enemy
        self.check_spawn_enemy().await;

        // Update/Draw all the enemies
        self.update_enemies();

        // Check for a collision between a bullet and an enemy
        self.check_bullet_and_enemy_collision().await;

        // Set the player character
        self.set_player_character().await;

        // Set the enemy character
        self.set_enemy_character().await;

        self.money.draw();
        self.bullet_count.draw();

        self.retreat_button.draw().await;
        self.retreat_button.update(
            vec2(screen_width() - 220., self.retreat_button.rect.y),
            self.retreat_button.rect.size(),
        );
    }

    async fn check_shoot_bullet(&mut self) {
        // Check if the left mouse button is pressed
        match is_mouse_button_down(MouseButton::Left) && self.gun.bullets > 0 {
            true => {
                // If the left mouse button is pressed, check if you can shoot a new bullet
                if self.gun.can_shoot {
                    self.gun.bullets -= 1;
                    self.bullet_count.decrement();

                    // Shoot a bullet
                    self.bullets.push(
                        Bullet::new(self.gun.rect.point() + self.gun.rect.size() * 0.5).await,
                    );

                    // Set the gun's can_shoot to false until you release the left mouse button
                    self.gun.can_shoot = false;

                    // Play the gunshot sound
                    play_sound(
                        load_sound("res/audio/shoot.wav").await.unwrap(),
                        PlaySoundParams {
                            volume: 0.3,
                            looped: false,
                        },
                    )
                }
            }
            false => {
                // Let the player shoot again
                self.gun.can_shoot = true;
            }
        }
    }

    fn update_bullets(&mut self) {
        // Loop through all bullets and draw/update them
        for bullet in &mut self.bullets {
            bullet.update(get_frame_time());
            bullet.draw();
        }

        for bullet in self.bullets.clone() {
            if bullet.rect.x > screen_width()
                || bullet.rect.y > screen_height()
                || bullet.rect.x < 0.
                || bullet.rect.y < 0.
            {
                self.bullets
                    .remove(self.bullets.iter().position(|r| *r == bullet).unwrap());
            }
        }
    }

    async fn check_spawn_enemy(&mut self) {
        if self.enemy_spawn_timer > 0. {
            self.enemy_spawn_timer -= 1.;
        } else {
            // Where the enemy will spawn
            enum Location {
                Top,
                Right,
                Bottom,
                Left,
            }

            let location = match rand::gen_range(1, 4) {
                1 => Location::Top,
                2 => Location::Right,
                3 => Location::Bottom,
                4 => Location::Left,
                _ => unreachable!(),
            };

            // Set to a random point as chosen location
            let x = match location {
                Location::Top => rand::gen_range(0., screen_width() as f32),
                Location::Right => screen_width() as f32,
                Location::Bottom => rand::gen_range(0., screen_width() as f32),
                Location::Left => 0.,
            };

            let y = match location {
                Location::Top => 0.,
                Location::Right => rand::gen_range(0., screen_height() as f32),
                Location::Bottom => screen_height() as f32,
                Location::Left => rand::gen_range(0., screen_height() as f32),
            };

            let pos = vec2(x, y);

            // Spawn enemy
            self.enemies
                .push(Enemy::new(pos, self.player.rect.point()).await);

            // Reset spawn timer
            self.enemy_spawn_timer = ENEMY_SPAWN_TIME
        }
    }

    fn update_enemies(&mut self) {
        // Loop through all enemies and draw/update them
        for enemy in &mut self.enemies {
            enemy.update(get_frame_time());
            enemy.draw();
        }

        for enemy in self.enemies.clone() {
            if enemy.rect.x > screen_width()
                || enemy.rect.y > screen_height()
                || enemy.rect.x < 0.
                || enemy.rect.y < 0.
            {
                self.enemies
                    .remove(self.enemies.iter().position(|r| *r == enemy).unwrap());
            }
        }
    }

    async fn check_bullet_and_enemy_collision(&mut self) {
        for enemy in self.enemies.clone() {
            for bullet in self.bullets.clone() {
                // Check if bullet collides with enemy
                if bullet.is_collision(enemy.clone())
                    && self.enemies.iter().position(|r| *r == enemy) != None
                {
                    // Remove enemy
                    self.enemies
                        .remove(self.enemies.iter().position(|r| *r == enemy).unwrap());
                    self.money.increment(rand::gen_range(
                        self.characters[self
                            .characters
                            .iter()
                            .position(|r| *r.0 == self.character.character)
                            .unwrap_or_default()]
                        .1 / 100,
                        self.characters[self
                            .characters
                            .iter()
                            .position(|r| *r.0 == self.character.character)
                            .unwrap_or_default()]
                        .1 / 10,
                    ));

                    // Play the enemy death sound
                    play_sound(
                        load_sound("res/audio/kill.wav").await.unwrap(),
                        PlaySoundParams {
                            volume: 0.3,
                            looped: false,
                        },
                    )
                }
            }
        }
    }

    async fn set_player_character(&mut self) {
        self.player.texture =
            "res/regular/".to_string() + self.character.character.to_lowercase().as_str() + ".png";
    }

    async fn set_enemy_character(&mut self) {
        for enemy in &mut self.enemies {
            enemy.texture = if self.character.character == "Apple".to_string() {
                load_texture("res/enemy/orange.png").await.unwrap()
            } else if self.character.character != "Banana".to_string() {
                load_texture(
                    ("res/enemy/".to_string()
                        + self.characters[self
                            .characters
                            .iter()
                            .position(|r| *r.0 == self.character.character)
                            .unwrap_or_default()
                            + 1]
                        .0
                        .to_lowercase()
                        .as_str()
                        + ".png")
                        .as_str(),
                )
                .await
                .unwrap()
            } else {
                load_texture("res/monkey.png").await.unwrap()
            };
        }
    }

    async fn check_reset(&mut self) {
        for enemy in self.enemies.clone() {
            if self.player.is_collision(enemy) {
                self.player.reset();
                self.gun
                    .reset(self.player.rect.point() + self.player.rect.size());

                self.enemies.clear();
                self.bullets.clear();

                self.money.reset();
                self.bullet_count.reset();

                self.game_state = GameState::Dead;

                self.gun.can_shoot = false;

                // Play the death sound
                play_sound(
                    load_sound("res/audio/die.wav").await.unwrap(),
                    PlaySoundParams {
                        volume: 0.5,
                        looped: false,
                    },
                )
            }
        }

        if self.retreat_button.is_pressed() {
            self.player.reset();
            self.gun
                .reset(self.player.rect.point() + self.player.rect.size());

            self.enemies.clear();
            self.bullets.clear();

            self.game_state = GameState::Upgrading;

            self.gun.can_shoot = false;
        }

        if self.gun.bullets <= 0 && self.money.money <= 0 && self.bullets.is_empty() {
            self.player.reset();
            self.gun
                .reset(self.player.rect.point() + self.player.rect.size());

            self.enemies.clear();
            self.bullets.clear();

            self.money.reset();
            self.bullet_count.reset();

            self.game_state = GameState::Dead;

            self.gun.can_shoot = false;

            self.out_of_bullets = true;
        }
    }
}

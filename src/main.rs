mod battling;
mod objects;
mod states;
mod upgrading;

use states::battling::Battling;
use states::dead::Dead;
use states::game_state::GameState;
use states::menu::Menu;
use states::tutorial::Tutorial;
use states::upgrading::Upgrading;

use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams};
use macroquad::prelude::*;

const MUSIC_TIME: f32 = 13.;

#[macroquad::main("Becoming The Banana Man")]
async fn main() {
    let mut game_state = GameState::Menu;

    let mut battling = Battling::new().await;
    let mut upgrading = Upgrading::new().await;
    let mut menu = Menu::new().await;
    let mut dead = Dead::new().await;
    let mut tutorial = Tutorial::new().await;

    let mut music_timer = 0.;

    let music = load_sound("res/audio/theme_song.wav").await.unwrap();

    loop {
        clear_background(GREEN);

        println!("{:?}", music_timer);

        match game_state {
            GameState::Battling => match battling.game_state {
                GameState::Battling => {
                    // Communicate data between upgrading and battling
                    upgrading.money = battling.money.clone();
                    upgrading.bullets = battling.bullet_count.clone();
                    battling.character = upgrading.character.clone();

                    if music_timer <= 0. {
                        play_sound(
                            music,
                            PlaySoundParams {
                                volume: 0.5,
                                looped: false,
                            },
                        );

                        music_timer = MUSIC_TIME;
                    } else {
                        music_timer -= 1. / get_fps() as f32;

                        println!("{}", music_timer);
                    }

                    battling.start().await;
                }

                _ => {
                    // Communicate game states
                    game_state = battling.game_state;
                    upgrading.game_state = battling.game_state;
                    dead.game_state = battling.game_state;
                }
            },

            GameState::Upgrading => match upgrading.game_state {
                GameState::Upgrading => {
                    // Communicate data between upgrading and battling
                    battling.money = upgrading.money.clone();
                    battling.bullet_count = upgrading.bullets.clone();

                    if music_timer <= 0. {
                        play_sound(
                            music,
                            PlaySoundParams {
                                volume: 0.5,
                                looped: false,
                            },
                        );

                        music_timer = MUSIC_TIME;
                    } else {
                        music_timer -= 1. / get_fps() as f32;
                    }

                    upgrading.start().await;
                }

                _ => {
                    // Communicate game states
                    battling.game_state = upgrading.game_state;
                    game_state = upgrading.game_state;
                }
            },

            GameState::Menu => {
                // Communicate game state
                tutorial.game_state = menu.game_state;
                game_state = menu.game_state;

                stop_sound(music);

                music_timer = 0.;

                menu.start().await;
            }

            GameState::Dead => {
                // Communicate game states
                menu.game_state = GameState::Menu;

                game_state = dead.game_state;
                battling.game_state = dead.game_state;

                // Communicate data
                dead.out_of_bullets = battling.out_of_bullets;

                stop_sound(music);

                music_timer = 0.;

                dead.start().await;
            }

            GameState::Tutorial => {
                // Communicate game states
                game_state = tutorial.game_state;
                menu.game_state = tutorial.game_state;

                tutorial.start().await;
            }
        }

        next_frame().await
    }
}

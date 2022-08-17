use imagesize::size;
use macroquad::prelude::*;
use once_cell::sync::Lazy;

// Gun size is equal to the size of the gun image
const GUN_SIZE: Lazy<Vec2> = Lazy::new(|| {
    const_vec2!([
        size("res/gun.png").unwrap().width as f32,
        size("res/gun.png").unwrap().height as f32,
    ])
});

pub struct Gun {
    pub rect: Rect,
    pub texture: Texture2D,
    pub can_shoot: bool,
    pub bullets: usize,
}

impl Gun {
    pub async fn new() -> Self {
        Self {
            // A rect will represent the gun bounds
            rect: Rect::new(
                screen_width() / 2. - GUN_SIZE[0] / 2.,
                screen_height() / 2. - GUN_SIZE[1] / 2.,
                GUN_SIZE[0],
                GUN_SIZE[1],
            ),
            texture: load_texture("res/gun.png").await.unwrap(),
            can_shoot: false,
            bullets: 0,
        }
    }

    pub fn update(&mut self, player_pos: Vec2) {
        self.rect.x = player_pos.x;
        self.rect.y = player_pos.y;
    }

    pub fn draw(&mut self) {
        // Draw the gun
        draw_texture_ex(
            self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                // Point the gun to the mouse
                rotation: (libm::atan2(
                    (mouse_position().1 - self.rect.y) as f64,
                    (mouse_position().0 - self.rect.x) as f64,
                ) * (180. / std::f64::consts::PI))
                    .to_radians() as f32,
                ..Default::default()
            },
        );
    }

    pub fn reset(&mut self, player_pos: Vec2) {
        self.bullets = 3;
        self.can_shoot = true;

        // Reset the rect position
        self.rect.x = player_pos.x;
        self.rect.y = player_pos.y;
    }
}

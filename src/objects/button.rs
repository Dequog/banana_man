use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    prelude::*,
};

pub struct Button {
    pub rect: Rect,
    pub text: String,
    font: Font,
    pub bg: Color,
    font_size: u16,

    can_click: bool,
}

impl Button {
    pub async fn new(pos: Vec2, size: Vec2, text: String, bg: Color, font_size: u16) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, size[0], size[1]),
            text,
            font: load_ttf_font("res/Roboto-Medium.ttf").await.unwrap(),
            bg,
            font_size,
            can_click: false,
        }
    }

    pub fn is_pressed(&mut self) -> bool {
        if is_mouse_button_down(MouseButton::Left)
            && self
                .rect
                .contains(vec2(mouse_position().0, mouse_position().1))
        {
            return true;
        }

        false
    }

    pub async fn draw(&mut self) {
        // Draw button background
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.bg);

        // Draw button text
        draw_text_ex(
            self.text.as_str(),
            self.rect.x + self.rect.w * 0.5 - (self.text.chars().count() - 1) as f32 * 7.5 * 0.5,
            self.rect.y + self.rect.h * 0.5 + 7.5,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                color: BLACK,
                font_scale: 1.,
                font_scale_aspect: 1.,
            },
        );

        // Play click sound if pressed
        if self.is_pressed() && self.can_click {
            play_sound(
                load_sound("res/audio/click.wav").await.unwrap(),
                PlaySoundParams {
                    volume: 0.5,
                    looped: false,
                },
            );

            self.can_click = false;
        } else if !self.is_pressed() {
            self.can_click = true;
        }
    }

    pub fn update(&mut self, pos: Vec2, size: Vec2) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
        self.rect.w = size[0];
        self.rect.h = size[1];
    }
}

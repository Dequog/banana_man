use macroquad::prelude::*;

#[derive(Clone)]
pub struct Text {
    pub font: Font,
    pub text: String,
    pub pos: Vec2,
    pub font_size: u16,
    pub color: Color,
}

impl Text {
    pub async fn new(pos: Vec2, font: String, text: String, font_size: u16, color: Color) -> Self {
        Self {
            font: load_ttf_font(font.as_str()).await.unwrap(),
            /* Text must be converted to a char array because a struct with a String
            field cannot implement the Copy trait*/
            text: text,
            pos,
            font_size,
            color,
        }
    }

    pub fn change(&mut self, text: String) {
        // Change the text
        self.text = text;
    }

    pub fn draw(&mut self) {
        // Draw the Text
        draw_text_ex(
            self.text.as_str(),
            self.pos.x,
            self.pos.y,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                color: self.color,
                font_scale: 1.,
                font_scale_aspect: 1.,
            },
        );
    }
}

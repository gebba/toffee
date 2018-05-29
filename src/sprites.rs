
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::{Texture, Canvas};

pub struct Sprite<'a> {
    rect: Rect,
    pub size: Rect,
    texture: &'a Texture,
}

impl<'a> Sprite<'a> {
    pub fn draw(&self, x: i32, y: i32, canvas: &mut Canvas<Window>) {
        let pos_rect = Rect::new(x, y, self.size.width(), self.size.height());
        match canvas.copy(&self.texture, Some(self.rect), Some(pos_rect)) {
            Err(e) => println!("canvas copy error: {}", e),
            _ => {}
        }
    }
}

pub struct SpriteSheet {
    pub sprite_width: u32,
    pub sprite_height: u32,
    padding: u32,
    pub texture: Texture,
}

impl SpriteSheet {
    pub fn new(texture: Texture, sprite_width: u32, sprite_height: u32, padding: u32) -> Self {
        SpriteSheet {
            sprite_width,
            sprite_height,
            padding,
            texture: texture,
        }
    }

    // Creates a sprite object
    pub fn get_sprite(&self, index: usize) -> Sprite {
        let texture_query = self.texture.query();
        let sheet_width = texture_query.width;

        let columns = sheet_width / (self.sprite_width + self.padding);

        let sheet_x = index as u32 % columns;
        let sheet_y = index as u32 / columns;

        let px = sheet_x * (self.sprite_width + self.padding);
        let py = sheet_y * (self.sprite_height + self.padding);

        Sprite {
            rect: Rect::new(px as i32, py as i32, self.sprite_width, self.sprite_height),
            size: Rect::new(0, 0, self.sprite_width, self.sprite_height),
            texture: &self.texture,
        }
    }
}

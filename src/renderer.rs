
use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::video::{Window, WindowContext};
use sdl2::render::TextureCreator;
use std::path::Path;
use terminal::{Terminal, Cell};
use font::FontDefinition;
use sprites::SpriteSheet;

pub struct Renderer {
    sdl_canvas: sdl2::render::Canvas<Window>,
    sprite_sheet: SpriteSheet,
    font: FontDefinition,
    dirty_optimization: bool,
}

impl Renderer {
    pub fn new(terminal: &Terminal, font: FontDefinition, dirty_optimization: bool) -> Self {

        let video_subsystem = terminal.sdl_context.video().unwrap();
        let window = video_subsystem.window("davokar-rl",
                    (terminal.columns * font.width) as u32,
                    (terminal.rows * font.height) as u32)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = window.into_canvas()
            .accelerated()
            .build()
            .unwrap();
        let texture_creator = sdl_canvas.texture_creator();

        let sprite_sheet = Renderer::load_font_sheet(&font, &texture_creator).unwrap();

        Renderer {
            sdl_canvas,
            sprite_sheet,
            font,
            dirty_optimization
        }
    }

    fn load_font_sheet(font: &FontDefinition,
                       texture_creator: &TextureCreator<WindowContext>)
                       -> Result<SpriteSheet, &'static str> {
        let texture_result = texture_creator.load_texture(Path::new(font.image_path));

        match texture_result {
            Ok(texture) => Ok(SpriteSheet::new(texture, font.width, font.height, font.padding)),
            Err(_) => Err("failed to load font texture"),
        }

    }

    pub fn draw(&mut self, terminal: &mut Terminal) {
        let total_cells = terminal.columns * terminal.rows;

        if !self.dirty_optimization {
            self.sdl_canvas.clear();
        }
        for i in 0..total_cells {
            self.draw_cell(i, terminal);
        }
        self.sdl_canvas.present();
    }

    fn draw_cell(&mut self, cell_index: u32, terminal: &mut Terminal) {
        let mut cell = &mut terminal.grid[cell_index as usize];

        if !cell.dirty && self.dirty_optimization {
            return; // TODO: not drawing all of the contents every frame makes one of the alternating back buffers empty, and results in flickering
        }

        self.sprite_sheet.texture.set_color_mod(cell.fg.r, cell.fg.g, cell.fg.b);
        let sprite = self.sprite_sheet.get_sprite((cell.glyph as u8) as usize);

        let x = cell_index % terminal.columns;
        let y = cell_index / terminal.columns;
        let px = x * self.font.width;
        let py = y * self.font.height;

        // draw the background for the cell
        self.sdl_canvas
            .set_draw_color(pixels::Color::RGBA(cell.bg.r,
                                                cell.bg.g,
                                                cell.bg.b,
                                                cell.bg.a));
        let _t = self.sdl_canvas
            .fill_rect(Rect::new(px as i32,
                                 py as i32,
                                 sprite.size.width(),
                                 sprite.size.height()));

        // draw the actual character
        sprite.draw(px as i32, py as i32, &mut self.sdl_canvas);
        cell.dirty = false;
    }
}

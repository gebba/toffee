
use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;
use std::path::Path;
use terminal::Terminal;
use font::FontDefinition;
use sprites::SpriteSheet;

pub struct Renderer<'a> {
    sdl_renderer: sdl2::render::Renderer<'a>,
    sprite_sheet: SpriteSheet,
    font: FontDefinition,
}

impl<'a> Renderer<'a> {
    pub fn new(terminal: &Terminal, font: FontDefinition) -> Self {

        let video_subsystem = terminal.sdl_context.video().unwrap();
        let window = video_subsystem.window("davokar-rl",
                    (terminal.columns * font.width) as u32,
                    (terminal.rows * font.height) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let sdl_renderer = window.renderer()
            .accelerated()
            // .present_vsync()
            .build()
            .unwrap();

        let sprite_sheet = Renderer::load_font_sheet(&font, &sdl_renderer).unwrap();

        Renderer {
            sdl_renderer: sdl_renderer,
            sprite_sheet: sprite_sheet,
            font: font,
        }
    }

    fn load_font_sheet(font: &FontDefinition,
                       renderer: &sdl2::render::Renderer)
                       -> Result<SpriteSheet, &'static str> {
        let texture_result = renderer.load_texture(Path::new(font.image_path));

        match texture_result {
            Ok(texture) => Ok(SpriteSheet::new(texture, font.width, font.height, font.padding)),
            Err(_) => Err("failed to load font texture"),
        }

    }

    pub fn draw(&mut self, terminal: &mut Terminal) {
        let total_cells = terminal.columns * terminal.rows;

        for i in 0..total_cells {
            let mut cell = &mut terminal.grid[i as usize];

            if cell.dirty {
                self.sprite_sheet.texture.set_color_mod(cell.fg.r, cell.fg.g, cell.fg.b);
                let sprite = self.sprite_sheet.get_sprite((cell.glyph as u8) as usize);

                let x = i % terminal.columns;
                let y = i / terminal.columns;
                let px = x * self.font.width;
                let py = y * self.font.height;

                // draw the background for the cell
                self.sdl_renderer
                    .set_draw_color(pixels::Color::RGBA(cell.bg.r,
                                                        cell.bg.g,
                                                        cell.bg.b,
                                                        cell.bg.a));
                let _t = self.sdl_renderer
                    .fill_rect(Rect::new(px as i32,
                                         py as i32,
                                         sprite.size.width(),
                                         sprite.size.height()));

                // draw the actual character
                sprite.draw(px as i32, py as i32, &mut self.sdl_renderer);
                cell.dirty = false;
            }
        }
        self.sdl_renderer.present();
    }
}
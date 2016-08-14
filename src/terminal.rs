extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2_image::{LoadTexture, INIT_PNG};

use sprites::SpriteSheet;
use colors::Color;

struct Cell {
    glyph: char,
    fg: Color,
    bg: Color,
}

pub struct FontDefinition {
    pub image_path: &'static str,
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    pub transparent: bool,
}

pub struct Terminal {
    sprite_sheet: SpriteSheet,
    grid: Vec<Cell>,
    renderer: Renderer<'static>,
    sdl_context: Sdl,
    font: FontDefinition,

    pub event_pump: EventPump,
    pub columns: u32,
    pub rows: u32,
}

impl Terminal {
    pub fn new(columns: u32, rows: u32, font: FontDefinition) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let _image_context = sdl2_image::init(INIT_PNG).unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("davokar-rl",
                    (columns * font.width) as u32,
                    (rows * font.height) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let sprite_sheet = Terminal::load_font_sheet(&font, &renderer).unwrap();

        let mut grid = vec![];

        let total_cells = columns * rows;
        for _ in 0..total_cells {
            grid.push(Cell {
                glyph: ' ',
                fg: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                bg: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            });
        }

        let event = sdl_context.event_pump().unwrap();

        Terminal {
            columns: columns,
            rows: rows,
            sprite_sheet: sprite_sheet,
            sdl_context: sdl_context,
            renderer: renderer,
            grid: grid,
            font: font,
            event_pump: event,
        }
    }

    fn load_font_sheet(font: &FontDefinition,
                       renderer: &Renderer)
                       -> Result<SpriteSheet, &'static str> {
        let texture_result = renderer.load_texture(Path::new(font.image_path));

        match texture_result {
            Ok(texture) => Ok(SpriteSheet::new(texture, font.width, font.height, font.padding)),
            Err(_) => Err("failed to load font texture"),
        }

    }

    pub fn set_cell(&mut self, x: i32, y: i32, c: char, fg: Color, bg: Color) {
        let index = self.columns as i32 * y + x;
        if x >= 0 && y >= 0 && x < self.columns as i32 && y < self.rows as i32 {
            self.grid[index as usize] = Cell {
                glyph: c,
                fg: fg,
                bg: bg,
            }
        }
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str, fg: Color, bg: Color) {
        let mut string_chars = text.chars();
        for i in 0..text.len() {
            let c = string_chars.next().unwrap();
            self.set_cell(x + i as i32, y, c, fg, bg);
        }
    }

    pub fn print_center(&mut self, y: i32, text: &str, fg: Color, bg: Color) {
        let center_x = self.columns / 2;
        let str_len_mid = text.len() / 2;
        let target_x = center_x - str_len_mid as u32;
        self.print(target_x as i32, y, text, fg, bg);
    }

    pub fn draw(&mut self) {
        self.renderer.clear();

        let total_cells = self.columns * self.rows;
        for i in 0..total_cells {
            let cell = &self.grid[i as usize];

            self.sprite_sheet.texture.set_color_mod(cell.fg.r, cell.fg.g, cell.fg.b);
            let sprite = self.sprite_sheet.get_sprite((cell.glyph as u8) as usize);

            let x = i % self.columns;
            let y = i / self.columns;
            let px = x * self.font.width;
            let py = y * self.font.height;

            // draw the background for the cell
            self.renderer
                .set_draw_color(pixels::Color::RGBA(cell.bg.r, cell.bg.g, cell.bg.b, cell.bg.a));
            let _t = self.renderer
                .fill_rect(Rect::new(px as i32,
                                     py as i32,
                                     sprite.size.width(),
                                     sprite.size.height()));

            // draw the actual character
            sprite.draw(px as i32, py as i32, &mut self.renderer);
        }
        self.renderer.present();
    }
}

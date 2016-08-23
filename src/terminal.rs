
use sdl2;
use sdl2_image;
use sdl2_image::INIT_PNG;
use colors::Color;

pub struct Cell {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

pub struct Terminal {
    pub grid: Vec<Cell>,
    pub columns: u32,
    pub rows: u32,
    pub sdl_context: sdl2::Sdl,
}

impl Terminal {
    pub fn new(columns: u32, rows: u32) -> Self {

        let sdl_context = sdl2::init().unwrap();
        let _image_context = sdl2_image::init(INIT_PNG).unwrap();

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

        Terminal {
            grid: grid,
            columns: columns,
            rows: rows,
            sdl_context: sdl_context,
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
}

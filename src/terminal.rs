use colors::Color;
use render::Renderer;

#[derive(Copy, Clone)]
pub struct Cell {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub dirty: bool,
}

impl Cell {
    pub fn new(glyph: char, fg: Color, bg: Color) -> Self {
        Cell {
            glyph,
            fg,
            bg,
            dirty: true,
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.glyph == other.glyph && self.fg == other.fg && self.bg == other.bg
    }
}

pub struct Terminal<'a> {
    pub grid: Vec<Cell>,
    pub columns: u32,
    pub rows: u32,
    pub renderer: Box<dyn Renderer + 'a>,
}

impl<'a> Terminal<'a> {
    pub fn new(renderer: Box<dyn Renderer + 'a>, columns: u32, rows: u32) -> Self {
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
                dirty: true,
            });
        }

        Terminal {
            grid,
            columns,
            rows,
            renderer,
        }
    }

    pub fn string_for_row(&self, row_index: u32) -> String {
        let mut result = String::new();
        for column_index in 0..self.columns {
            match self.get_cell(column_index as i32, row_index as i32) {
                Some(cell) => result.push(cell.glyph),
                None => {}
            }
        }
        result
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        let index = self.columns as i32 * y + x;
        if x >= 0 && y >= 0 && x < self.columns as i32 && y < self.rows as i32 {
            Some(&self.grid[index as usize])
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.columns as i32 * y + x;
        if x >= 0 && y >= 0 && x < self.columns as i32 && y < self.rows as i32 {
            let current_cell = self.grid[index as usize];
            if current_cell != cell {
                self.grid[index as usize] = cell;
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, c: char, fg: Color, bg: Color) {
        let cell = Cell {
            glyph: c,
            fg,
            bg,
            dirty: true,
        };
        self.set_cell(x, y, cell);
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str, fg: Color, bg: Color) {
        let mut string_chars = text.chars();
        for i in 0..text.len() {
            let c = string_chars.next().unwrap();
            self.set(x + i as i32, y, c, fg, bg);
        }
    }

    pub fn print_center(&mut self, y: i32, text: &str, fg: Color, bg: Color) {
        let center_x = self.columns / 2;
        let str_len_mid = text.len() / 2;
        let target_x = center_x - str_len_mid as u32;
        self.print(target_x as i32, y, text, fg, bg);
    }

    pub fn render(&self) {
        self.renderer.render(self);
    }
}

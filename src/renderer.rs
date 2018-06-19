
use gfx;
use glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;
use gfx_window_glutin as gfx_glutin;
use std::path::Path;
use terminal::{Cell, Terminal};
use font::FontDefinition;
use sprites::SpriteSheet;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct Renderer {
    gl_window: GlWindow,
    sprite_sheet: SpriteSheet,
    texture_creator: TextureCreator<WindowContext>,
    font: FontDefinition,
}

impl Renderer {
    pub fn new(terminal: &Terminal, font: FontDefinition) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_title("davokar-rl")
            .with_dimensions(terminal.columns * font.width, terminal.rows * font.height);
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl, (3,2)))
            .with_vsync(true);

        let (window, mut device, mut factory, color_view, mut depth_view) = 
            gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

        let sprite_sheet = Renderer::load_font_sheet(&font, &texture_creator).unwrap();

        Renderer {
            gl_window,
            sprite_sheet,
            texture_creator: texture_creator,
            font,
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

        for i in 0..total_cells {
            let mut cell = &mut terminal.grid[i as usize];

            if cell.dirty {
                self.sprite_sheet.texture.set_color_mod(cell.fg.r, cell.fg.g, cell.fg.b);
                let sprite = self.sprite_sheet.get_sprite((cell.glyph as u8) as usize);

                let x = i % terminal.columns;
                let y = i / terminal.columns;
                let px = x * self.font.width;
                let py = y * self.font.height;

                self.draw_cell(cell);

                cell.dirty = false;
            }
        }
        self.sdl_canvas.present();
    }

    fn draw_cell(&mut self, cell: &mut Cell) {
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
    }
}

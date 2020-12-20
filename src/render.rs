mod wgpu;

pub use render::wgpu::WgpuRenderer;

use terminal::Terminal;

pub trait Renderer {
    fn render(&self, term: &Terminal);
}

pub struct DebugRenderer {}

impl Renderer for DebugRenderer {
    fn render(&self, terminal: &Terminal) {
        let border_string = std::iter::repeat("#")
            .take(terminal.columns as usize + 2)
            .collect::<String>();
        println!("{}", border_string);
        for row in 0..terminal.rows {
            println!("#{}#", terminal.string_for_row(row));
        }
        println!("{}", border_string);
    }
}

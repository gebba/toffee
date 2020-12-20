extern crate toffee;

use toffee::colors;
//use toffee::font::FontDefinition;
use toffee::render::DebugRenderer;
use toffee::terminal::Terminal;

pub fn main() {
    /*
    let font = FontDefinition {
        image_path: "fonts/grim_10x12.png",
        width: 10,
        height: 12,
        padding: 0,
        transparent: false,
    };
     */
    let renderer = Box::new(DebugRenderer {});

    let mut term = Terminal::new(renderer, 50, 20);
    term.print_center(5, "It seems to be working!", colors::WHITE, colors::BLUE);
    term.render();
}

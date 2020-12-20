extern crate toffee;

use std::thread::sleep;
use std::time::Duration;
use toffee::colors;
use toffee::render::WgpuRenderer;
use toffee::terminal::Terminal;

pub fn main() {
    let renderer = Box::new(WgpuRenderer::new());
    let mut term = Terminal::new(renderer, 50, 20);

    term.print_center(5, "It seems to be working!", colors::WHITE, colors::BLUE);
    term.render();
    renderer.run();
}

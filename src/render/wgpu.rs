use super::Renderer;
use super::Terminal;

use winit::event_loop::EventLoopWindowTarget;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct WgpuRenderer {
    window: Window,
    event_loop: EventLoop<()>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        WgpuRenderer { window, event_loop }
    }

    pub fn run(&self) {
        self.event_loop
            .run(move |event, window_target, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            });
    }
}

impl Renderer for WgpuRenderer {
    fn render(&self, terminal: &Terminal) {}
}

use wgpu::Device;
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

struct State {
}

impl State {
    fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

        let surface = unsafe { instance.create_surface(window) };

        Self {}
    }

    fn render(&self) {

    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let state = State::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                state.render();
            }
            _ => (),
        }
    });
}
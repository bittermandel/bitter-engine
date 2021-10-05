use anyhow::Result;

use camera::{controller::CameraController, projection::Projection};
use wgpu::Device;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
mod camera;
mod triangle;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    queue: wgpu::Queue,
    pub entities: Vec<triangle::Triangle>,
    pub camera: camera::Camera,
    pub camera_controller: CameraController,
}

impl State {
    async fn new(window: &Window) -> State {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::DEPTH_CLAMPING
                        | wgpu::Features::TEXTURE_BINDING_ARRAY,
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            format: surface.get_preferred_format(&adapter).unwrap(),
            height: window.inner_size().height,
            width: window.inner_size().width,
            present_mode: wgpu::PresentMode::Mailbox,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        surface.configure(&device, &config);

        let entities: Vec<triangle::Triangle> = Vec::new();

        let projection =
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera = camera::Camera::new(
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
            projection,
            &device,
        );

        let camera_controller = CameraController::new(4.0, 0.4);

        camera.calc_matrix();

        Self {
            surface,
            device,
            config,
            queue,
            entities,
            camera,
            camera_controller,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_frame()?.output;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        let camera = self.camera.get_bind_group(&self.device);

        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            for entity in &self.entities {
                entity.render(&mut _render_pass, &camera);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = pollster::block_on(State::new(&window));
    let vert_shader_desc = wgpu::include_spirv!("../shaders/triangle.vert.spv");
    let frag_shader_desc = wgpu::include_spirv!("../shaders/triangle.frag.spv");

    state.entities.push(triangle::Triangle::new(
        0.0,
        &state.device,
        &state.config,
        &vert_shader_desc,
        &frag_shader_desc,
        &state.camera.bind_group_layout,
    ));

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

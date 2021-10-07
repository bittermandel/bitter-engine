use anyhow::Result;

use camera::{controller::CameraController, projection::Projection};

use winit::{
    event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::model::{render::DrawModel, Material};
mod camera;
mod lights;
mod model;
mod pipeline;
mod texture;
mod triangle;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    queue: wgpu::Queue,
    pub entities: Vec<model::Model>,
    pub camera: camera::Camera,
    pub camera_controller: CameraController,
    pub directional_light: lights::Light,
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

        let entities: Vec<model::Model> = Vec::new();

        let projection =
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera = camera::Camera::new(
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
            projection,
            &device,
        );
        let directional_light = lights::Light::new(&device);

        let camera_controller = CameraController::new(4.0, 0.4);

        Self {
            surface,
            device,
            config,
            queue,
            entities,
            camera,
            camera_controller,
            directional_light,
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

        let camera_bind_group = &self.camera.get_bind_group(&self.device);
        let dir_light_bind_group = &self.directional_light.get_bind_group(&self.device);

        let pipeline = &pipeline::opaque_render_pipeline(
            &self.device,
            &self.config,
            &[
                &texture::Texture::bind_group_layout(&self.device),
                &self.camera.bind_group_layout,
                &self.directional_light.bind_group_layout,
            ],
        );

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

            _render_pass.set_pipeline(pipeline);
            _render_pass.set_bind_group(1, camera_bind_group, &[]);
            _render_pass.set_bind_group(2, dir_light_bind_group, &[]);

            for entity in &self.entities {
                _render_pass.draw_mesh(&entity.meshes[0], &entity.materials[0]);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    fn update(&mut self, dt: std::time::Duration) {
        // UPDATED!
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera.update_view_proj();

        // ..
    }

    // UPDATED!
    fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => self.camera_controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(&*delta);
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                self.camera_controller.process_mouse(delta.0, delta.1);
                true
            }
            _ => false,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_cursor_grab(true);

    let res_dir = std::path::Path::new(env!("OUT_DIR")).join("res");

    let mut state = pollster::block_on(State::new(&window));

    state.entities.push(
        model::Model::load(
            &state.device,
            &state.queue,
            &texture::Texture::bind_group_layout(&state.device),
            res_dir.join("cube.obj"),
        )
        .unwrap(),
    );

    let mut last_render_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent {
                ref event,
                .. // We're not using device_id currently
            } => {
                state.input(event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    });
}

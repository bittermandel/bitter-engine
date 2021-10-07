use wgpu::util::{DeviceExt, RenderEncoder};

use crate::camera::{self, Camera};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Triangle {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl<'a> Triangle {
    pub fn new(
        offset: f32,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        vert_shader_desc: &wgpu::ShaderModuleDescriptor,
        frag_shader_desc: &wgpu::ShaderModuleDescriptor,
        camera_bind_layout: &wgpu::BindGroupLayout,
        light_bind_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let vert_shader = device.create_shader_module(vert_shader_desc);
        let frag_shader = device.create_shader_module(frag_shader_desc);

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("triangle pipeline layout"),
            bind_group_layouts: &[camera_bind_layout, light_bind_layout],
            ..Default::default()
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &frag_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&[
                Vertex {
                    position: [0.0 + offset, 0.5 + offset, 0.0 + offset],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5 + offset, -0.5 + offset, 0.0 + offset],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5 + offset, -0.5 + offset, 0.0 + offset],
                    color: [0.0, 0.0, 1.0],
                },
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Triangle {
            pipeline,
            vertex_buffer,
        }
    }

    pub fn render(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_bind_group: &'a wgpu::BindGroup,
        dir_light_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_bind_group(1, dir_light_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

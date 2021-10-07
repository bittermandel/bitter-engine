use wgpu::DepthStencilState;

use crate::{
    model::{self, Vertex},
    texture,
};

pub fn opaque_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let vert_shader =
        device.create_shader_module(&wgpu::include_spirv!("../shaders/opaque.vert.spv"));
    let frag_shader =
        device.create_shader_module(&wgpu::include_spirv!("../shaders/opaque.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vert_shader,
            entry_point: "main",
            buffers: &[model::ModelVertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLAMPING
            clamp_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None, /*Some(wgpu::DepthStencilState {
                                 format: wgpu::TextureFormat::Depth32Float,
                                 depth_write_enabled: true,
                                 depth_compare: wgpu::CompareFunction::Less,
                                 stencil: wgpu::StencilState::default(),
                                 bias: wgpu::DepthBiasState::default(),
                             }),*/
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    })
}

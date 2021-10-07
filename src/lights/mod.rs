use cgmath::vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 3],
    direction: [f32; 3],
}

pub struct Light {
    uniform: LightUniform,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Light {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform = LightUniform {
            position: [0.0, 10.0, 10.0],
            direction: [-0.2, -1.0, -0.3],
        };
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("light_bind_group_layout"),
        });

        Self {
            uniform,
            bind_group_layout,
        }
    }

    pub fn get_bind_group<'a>(&mut self, device: &wgpu::Device) -> wgpu::BindGroup {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("light_buffer"),
            contents: bytemuck::cast_slice(&[self.uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("light_bind_group"),
        })
    }
}

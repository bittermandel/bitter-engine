use cgmath::{InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use self::projection::Projection;

pub mod controller;
pub mod projection;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
    pub projection: Projection,
    pub uniform: CameraUniform,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
        projection: Projection,
        device: &wgpu::Device,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            projection,
            uniform: CameraUniform {
                view_position: [0.0; 4],
                view_proj: Matrix4::identity().into(),
            },
            bind_group_layout: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("camera_bind_group_layout"),
            }),
        }
    }

    pub fn update_view_proj(&mut self) {
        self.uniform.view_position = self.position.to_homogeneous().into();
        self.uniform.view_proj = (self.projection.calc_matrix() * self.calc_matrix()).into();
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(self.yaw.0.cos(), self.pitch.0.sin(), self.yaw.0.sin()).normalize(),
            Vector3::unit_y(),
        )
    }

    pub fn get_bind_group<'a>(&mut self, device: &wgpu::Device) -> wgpu::BindGroup {
        self.update_view_proj();
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[self.uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        })
    }
}

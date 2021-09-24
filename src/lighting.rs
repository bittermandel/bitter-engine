use std::ops::Range;

use crate::{camera::OPENGL_TO_WGPU_MATRIX, model::{Mesh, Model}};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightRaw{
    pub proj: [[f32; 4]; 4],
    pub position: [f32; 4],
    pub color: [f32; 4],
}

pub struct Light{
    pub position: cgmath::Point3<f32>,
}

impl Light {
    pub fn to_raw(&self) -> LightRaw {
        use cgmath::{Deg, EuclideanSpace, Matrix4, PerspectiveFov, Point3, Vector3};

        let mx_view = Matrix4::look_at_rh(self.position, Point3::origin(), Vector3::unit_z());
        let projection = PerspectiveFov {
            fovy: Deg(45.0).into(),
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
        };
        let mx_correction = OPENGL_TO_WGPU_MATRIX;
        let mx_view_proj =
            cgmath::Matrix4::from(projection.to_perspective()) * mx_view;
        LightRaw {
            proj: *mx_view_proj.as_ref(),
            position: [self.position.x, self.position.y, self.position.z, 1.0],
            color: [
                1.0,
                1.0,
                1.0,
                1.0,
            ],
        }
    }
}

pub trait DrawLight<'a> {
    fn draw_light_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: Range<u32>,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );

    fn draw_light_model(
        &mut self,
        model: &'a Model,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_light_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_light_mesh(
        &mut self,
        mesh: &'b Mesh,
        camera: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.draw_light_mesh_instanced(mesh, 0..1, camera, light);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: Range<u32>,
        camera: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera, &[]);
        self.set_bind_group(1, light, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'b Model,
        camera: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        self.draw_light_model_instanced(model, 0..1, camera, light);
    }
    fn draw_light_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera: &'b wgpu::BindGroup,
        light: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(mesh, instances.clone(), camera, light);
        }
    }
}

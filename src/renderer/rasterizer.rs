use glam::Mat4;
use wgpu::util::DeviceExt;

use crate::{
    WGPUBackend,
    logic::play::Play,
    renderer::{
        pipeline,
        pipeline::ColorVertex
    }
};

pub struct TestRasterizer {
    pipeline: pipeline::ColorPipeline,

    projection_buffer: wgpu::Buffer,
    view_buffer: wgpu::Buffer,
    model_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl TestRasterizer {
    pub fn new(wgpu_backend: &WGPUBackend, play: &Play) -> Self {
        let pipeline = pipeline::ColorPipeline::new(wgpu_backend);

        let projection_data = play.camera.get_projection_matrix((wgpu_backend.config.width, wgpu_backend.config.height));
        let projection_ref: &[f32; 16] = projection_data.as_ref();
        let projection_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(projection_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let view_data = play.camera.get_view_matrix();
        let view_ref: &[f32; 16] = view_data.as_ref();
        let view_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(view_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_data = Mat4::IDENTITY;
        let model_ref: &[f32; 16] = model_data.as_ref();
        let model_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(model_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = wgpu_backend.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: view_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: model_buffer.as_entire_binding(),
                },
            ],
        });

        // Back culled green cube
        let mut vertices = Vec::<ColorVertex>::new();
        vertices.push(ColorVertex { position: [0.0, 0.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [200.0, 0.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [0.0, 100.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [200.0, 100.0], color: [0, 255, 0, 255] });

        let vertex_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let index_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            pipeline,

            projection_buffer,
            view_buffer,
            model_buffer,

            bind_group,

            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        };
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let projection_data = play.camera.get_projection_matrix((wgpu_backend.config.width, wgpu_backend.config.height));
        let projection_ref: &[f32; 16] = projection_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.projection_buffer, 0, bytemuck::cast_slice(projection_ref));

        let view_data = play.camera.get_view_matrix();
        let view_ref: &[f32; 16] = view_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.view_buffer, 0, bytemuck::cast_slice(view_ref));
    }
}
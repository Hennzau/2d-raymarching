use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use crate::logic::play::Play;
use crate::renderer::pipeline;
use crate::renderer::pipeline::ColorVertex;

use crate::WGPUBackend;

pub mod world;

pub struct PlayRenderer {
    pipeline: pipeline::ColorPipeline,

    projection_buffer: wgpu::Buffer,
    view_buffer: wgpu::Buffer,
    model_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    world: world::WorldRenderer,
}

impl PlayRenderer {
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

        let model_data = Mat4::from_translation(Vec3::new(140.0, 60.0, 0.0));
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

        let world = world::WorldRenderer::new(wgpu_backend, &play.world);

        return Self {
            pipeline,

            projection_buffer,
            view_buffer,
            model_buffer,

            bind_group,
            world
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let view_data = play.camera.get_view_matrix();
        let view_ref: &[f32; 16] = view_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.view_buffer, 0, bytemuck::cast_slice(view_ref));
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let projection_data = play.camera.get_projection_matrix((wgpu_backend.config.width, wgpu_backend.config.height));
        let projection_ref: &[f32; 16] = projection_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.projection_buffer, 0, bytemuck::cast_slice(projection_ref));
    }


    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, play: &Play) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        self.world.render(pass);
    }
}
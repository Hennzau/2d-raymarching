use glam::{Mat4, Vec3};
use wgpu::BindingResource::BufferArray;
use wgpu::{BufferBinding, BufferSize};
use wgpu::util::DeviceExt;
use crate::logic::play::Play;
use crate::renderer::pipeline;
use crate::renderer::pipeline::ColorVertex;

use crate::WGPUBackend;

pub mod world;

pub struct PlayRenderer {
    pipeline: pipeline::RayMarchingPipeline,

    mvp_buffer: wgpu::Buffer,
    inverted_mvp_buffer: wgpu::Buffer,
    world_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    world: world::WorldRenderer,
}

impl PlayRenderer {
    pub fn new(wgpu_backend: &WGPUBackend, play: &Play) -> Self {
        let pipeline = pipeline::RayMarchingPipeline::new(wgpu_backend);

        let mvp_data = play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model;
        let mvp_ref: &[f32; 16] = mvp_data.as_ref();
        let mvp_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let inverted_mvp_data = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model).inverse();
        let inverted_mvp_ref: &[f32; 16] = inverted_mvp_data.as_ref();
        let inverted_mvp_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(inverted_mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let world_data = play.world.tiles;
        let world_ref = play.world.tiles.as_ref();
        let world_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(world_ref),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = wgpu_backend.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: mvp_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: inverted_mvp_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: world_buffer.as_entire_binding(),
                },
            ],
        });

        let world = world::WorldRenderer::new(wgpu_backend, &play.world);

        return Self {
            pipeline,

            mvp_buffer,
            inverted_mvp_buffer,
            world_buffer,

            bind_group,
            world,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let mvp_data = play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model;
        let mvp_ref: &[f32; 16] = mvp_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(mvp_ref));

        let inverted_mvp_data = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model).inverse();
        let inverted_mvp_ref: &[f32; 16] = inverted_mvp_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.inverted_mvp_buffer, 0, bytemuck::cast_slice(inverted_mvp_ref));
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let mvp_data = play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model;
        let mvp_ref: &[f32; 16] = mvp_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(mvp_ref));

        let inverted_mvp_data = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height)) * play.model).inverse();
        let inverted_mvp_ref: &[f32; 16] = inverted_mvp_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.inverted_mvp_buffer, 0, bytemuck::cast_slice(inverted_mvp_ref));
    }


    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, play: &Play) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        self.world.render(pass);
    }
}
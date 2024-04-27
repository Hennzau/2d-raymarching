use glam::{
    Vec2,
    Vec3,
    Vec4
};

use wgpu::util::DeviceExt;

use crate::{
    logic::play::Play,
    renderer::pipeline
};

use crate::WGPUBackend;

pub mod world;

pub struct PlayRenderer {
    pipeline: pipeline::RayMarchingPipeline,

    inverted_mvp_buffer: wgpu::Buffer,
    surface_configuration_buffer: wgpu::Buffer,
    point_light_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    world: world::WorldRenderer,
}

impl PlayRenderer {
    pub fn new(wgpu_backend: &WGPUBackend, play: &Play) -> Self {
        let pipeline = pipeline::RayMarchingPipeline::new(wgpu_backend);

        let inverted_mvp_data = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height))).inverse();
        let inverted_mvp_ref: &[f32; 16] = inverted_mvp_data.as_ref();
        let inverted_mvp_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(inverted_mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let surface_configuration_data = [wgpu_backend.config.width as f32, wgpu_backend.config.height as f32];
        let surface_configuration_ref = surface_configuration_data.as_ref();
        let surface_configuration_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(surface_configuration_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let point_light_data = play.mouse_position;
        let point_light_ref = point_light_data.as_ref();
        let point_light_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(point_light_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

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
                    resource: inverted_mvp_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: surface_configuration_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: point_light_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: world_buffer.as_entire_binding(),
                },
            ],
        });

        let world = world::WorldRenderer::new(wgpu_backend, &play.world);

        return Self {
            pipeline,

            inverted_mvp_buffer,
            surface_configuration_buffer,
            point_light_buffer,

            bind_group,
            world,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let x = 2.0 * play.mouse_position.x / wgpu_backend.config.width as f32 - 1.0;
        let y = 1.0 - (2.0 * play.mouse_position.y) / wgpu_backend.config.height as f32;
        let z = 1.0f32;

        let ray_nds = Vec3::new(x, y, z);
        let ray_clip = Vec4::new(ray_nds.x, ray_nds.y, -1.0, 1.0);

        let ray_eye = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height))).inverse() * ray_clip;

        let point_light_data = Vec2::new(ray_eye.x, ray_eye.y);
        let point_light_ref = point_light_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.point_light_buffer, 0, bytemuck::cast_slice(point_light_ref));
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let inverted_mvp_data = (play.camera.mvp((wgpu_backend.config.width, wgpu_backend.config.height))).inverse();
        let inverted_mvp_ref: &[f32; 16] = inverted_mvp_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.inverted_mvp_buffer, 0, bytemuck::cast_slice(inverted_mvp_ref));

        let surface_configuration_data = [wgpu_backend.config.width as f32, wgpu_backend.config.height as f32];
        let surface_configuration_ref = surface_configuration_data.as_ref();
        wgpu_backend.queue.write_buffer(&self.surface_configuration_buffer, 0, bytemuck::cast_slice(surface_configuration_ref));
    }


    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, _play: &Play) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        self.world.render(pass);
    }
}
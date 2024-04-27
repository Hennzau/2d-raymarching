use wgpu::util::DeviceExt;
use crate::logic::play::world::World;
use crate::renderer::pipeline::{ColorVertex, SimpleVertex};

use crate::WGPUBackend;

pub struct WorldRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl WorldRenderer {
    pub fn new(wgpu_backend: &WGPUBackend, world: &World) -> Self {
        let mut vertices = Vec::<ColorVertex>::new();
        let mut indices = Vec::<u16>::new();

        let mut vertices = Vec::<SimpleVertex>::new();
        vertices.push(SimpleVertex { position: [-1.0, 1.0] });
        vertices.push(SimpleVertex { position: [-1.0, -1.0] });
        vertices.push(SimpleVertex { position: [1.0, -1.0] });
        vertices.push(SimpleVertex { position: [1.0, 1.0] });

        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

        let vertex_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        };
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
use wgpu::{
    LoadOp,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor,
};

use crate::{
    logic::Logic,
    WGPUBackend
};

use crate::renderer::play::PlayRenderer;

pub mod pipeline;

pub mod play;

pub struct Renderer {
    play: PlayRenderer
}

impl Renderer {
    pub fn new(wgpu_backend: &WGPUBackend, logic: &Logic) -> Self {
        return Self {
            play: PlayRenderer::new(wgpu_backend, &logic.play)
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        self.play.update(wgpu_backend, &logic.play);
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        self.play.process_resize(wgpu_backend, &logic.play);
    }

    pub fn render(&self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        let frame = wgpu_backend.surface.get_current_texture().expect("Failed to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = wgpu_backend.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.play.render(&mut pass, &logic.play);
        }

        wgpu_backend.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
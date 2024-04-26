use std::{
    borrow::Cow,
    mem,
};

use bytemuck::{
    Pod,
    Zeroable,
};

use wgpu::{
    BindGroupLayout,
    Face,
    RenderPipeline,
};

use crate::WGPUBackend;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 2],
    pub color: [u8; 4],
}

pub struct ColorPipeline {
    pub layout: BindGroupLayout,
    pub pipeline: RenderPipeline,
}

impl ColorPipeline {
    pub fn new(wgpu_backend: &WGPUBackend) -> Self {
        let bind_group_layout = wgpu_backend.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout for ColorPipeline"),
            entries: &[
                wgpu::BindGroupLayoutEntry { // Projection * View * Model Matrix
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
            ],
        });

        let shader = wgpu_backend.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/color.wgsl"))),
        });

        let pipeline_layout = wgpu_backend.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_size = mem::size_of::<ColorVertex>();

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: 2 * 4,
                    shader_location: 1,
                }
            ],
        };

        let render_pipeline = wgpu_backend.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu_backend.config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        return Self {
            layout: bind_group_layout,
            pipeline: render_pipeline,
        };
    }
}

pub struct RayMarchingPipeline {
    pub layout: BindGroupLayout,
    pub pipeline: RenderPipeline,
}

impl RayMarchingPipeline {
    pub fn new(wgpu_backend: &WGPUBackend) -> Self {
        let bind_group_layout = wgpu_backend.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout for RayMarchingPipeline"),
            entries: &[
                wgpu::BindGroupLayoutEntry { // MVP
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Inverted MVP
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Tiles
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(1500 * 4),
                    },
                    count: None,
                },
            ],
        });

        let shader = wgpu_backend.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/ray_marching.wgsl"))),
        });

        let pipeline_layout = wgpu_backend.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_size = mem::size_of::<ColorVertex>();

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: 2 * 4,
                    shader_location: 1,
                }
            ],
        };

        let render_pipeline = wgpu_backend.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu_backend.config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        return Self {
            layout: bind_group_layout,
            pipeline: render_pipeline,
        };
    }
}
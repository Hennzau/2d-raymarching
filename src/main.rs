use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::EventLoop,
    window::{
        Window,
        WindowBuilder,
    },
    dpi::{
        LogicalSize,
        PhysicalPosition,
    },
};

use wgpu::{
    Adapter,
    Device,
    Instance,
    Queue,
    Surface,
    SurfaceConfiguration,
    TextureFormat,
};

use crate::logic::Logic;
use crate::renderer::Renderer;

async fn build_backend(window: &Window) -> (Instance, Surface, SurfaceConfiguration, Adapter, Device, Queue) {
    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(window).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await.expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
        },
        None,
    ).await.expect("Failed to create device");

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let mut config = surface.get_default_config(&adapter, size.width, size.height).unwrap();

    config.format = TextureFormat::Bgra8Unorm;

    surface.configure(&device, &config);

    return (instance, surface, config, adapter, device, queue);
}

pub struct WGPUBackend<'a> {
    instance: Instance,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

fn build_wgpu_backed(window: &Window) -> WGPUBackend {
    let (instance, surface, config, adapter, device, queue) = pollster::block_on(build_backend(&window));

    return WGPUBackend {
        instance,
        surface,
        config,
        adapter,
        device,
        queue,
    };
}

pub mod logic;
pub mod renderer;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window = builder.with_title("Vox").with_inner_size(LogicalSize::new(1280, 720)).build(&event_loop).unwrap();

    if let Some(monitor) = window.current_monitor() {
        let screen_size = monitor.size();
        let window_size = window.outer_size();

        window.set_outer_position(PhysicalPosition {
            x: screen_size.width.saturating_sub(window_size.width) as f64 / 2. + monitor.position().x as f64,
            y: screen_size.height.saturating_sub(window_size.height) as f64 / 2. + monitor.position().y as f64,
        });
    }

    let mut backend = build_wgpu_backed(&window);

    let mut logic = Logic::new();
    let mut renderer = Renderer::new(&backend, &logic);

    let window = &window;
    event_loop.run(move |event, target| {
        let _ = (&backend.instance, &backend.adapter);

        match event {
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        backend.config.width = new_size.width.max(1);
                        backend.config.height = new_size.height.max(1);

                        backend.surface.configure(&backend.device, &backend.config);
                        renderer.process_resize(&backend, &logic);
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::RedrawRequested => {
                        renderer.update(&backend, &logic);
                        renderer.render(&backend, &logic);
                    }
                    WindowEvent::CursorMoved {
                        position,
                        ..
                    } => {
                        logic.process_mouse_position((position.x as u32, position.y as u32));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).expect("TODO: panic message");
}
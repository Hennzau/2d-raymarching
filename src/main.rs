use std::{
    thread::sleep,
    time::Duration,
};

use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::EventLoop,
    platform::pump_events::{
        EventLoopExtPumpEvents,
        PumpStatus,
    },
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
use winit::event::DeviceEvent;

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
            required_limits: wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
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
    let mut event_loop = EventLoop::new().unwrap();
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

    'main: loop {
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_events(timeout, |event, target| {
            match event {
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
                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } => logic.process_keyboard(&window, event),
                        WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        } => logic.process_mouse_input(&window, state, button),
                        _ => {}
                    }
                }
                Event::DeviceEvent {
                    event,
                    ..
                } => {
                    match event {
                        DeviceEvent::MouseMotion {
                            delta: (dx, dy)
                        } => logic.process_mouse_motion((dx as f32, dy as f32)),
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        if let PumpStatus::Exit(_) = status {
            break 'main;
        }

        renderer.render(&backend, &logic);

        logic.update(1.0 / 60.0);
        renderer.update(&backend, &logic);

        sleep(Duration::from_millis(16)); // At the moment we just put everything at 60 ticks/per_second
    }
}
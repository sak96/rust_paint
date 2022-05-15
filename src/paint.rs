use crate::canvas::Canvas;
use crate::event::InputHandler;
use wgpu::{
    Backends, DeviceDescriptor, Features, Instance, Limits, PowerPreference, RequestAdapterOptions,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub fn run(event_loop: EventLoop<()>, window: Window) {
    let mut input = WinitInputHelper::new();
    let size = window.inner_size();
    let instance = Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = futures::executor::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .expect("Failed to find an appropriate adapter");

    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &DeviceDescriptor {
            label: None,
            features: Features::PUSH_CONSTANTS,
            limits: Limits {
                max_push_constant_size: 32,
                ..Limits::default()
            },
        },
        None,
    ))
    .expect("Failed to create device");
    let mut input_handler = InputHandler::new();
    let mut canvas = Canvas::new(size, surface, device, adapter, queue);
    event_loop.run(move |event, _, control_flow| {
        let _ = &instance;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                canvas.resize_window(new_size);
            }
            Event::RedrawRequested(_) => {
                canvas.redraw_canvas();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                if input.update(&event) {
                    let redraw_window = input_handler.handle_input(&input, &mut canvas);
                    if redraw_window {
                        window.request_redraw();
                    }
                }
            }
        }
    });
}

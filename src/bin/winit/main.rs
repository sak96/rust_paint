#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::float_cmp)]

use rusty_paint::canvas::{Canvas, PhysicalSize};
use wgpu::{DeviceDescriptor, Features, Limits, PowerPreference, RequestAdapterOptions};

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorIcon, Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;
fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    run(event_loop, window);
}

pub fn run(event_loop: EventLoop<()>, window: Window) {
    let mut input = WinitInputHelper::new();
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
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
    let mut canvas = Canvas::new(
        PhysicalSize {
            width: size.width,
            height: size.width,
        },
        surface,
        device,
        adapter,
        queue,
    );
    event_loop.run(move |event, _, control_flow| {
        let _ = &instance;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                canvas.resize_window(PhysicalSize {
                    width: size.width,
                    height: size.width,
                });
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
                    let redraw_window = handle_input(&input, &window, &mut canvas);
                    if redraw_window {
                        window.request_redraw();
                    }
                }
            }
        }
    });
}

fn handle_input(input: &WinitInputHelper, window: &Window, canvas: &mut Canvas) -> bool {
    let mut redraw_window = false;
    if input.key_pressed(VirtualKeyCode::Plus) {
        canvas.inc_brush_size();
    }
    if input.key_pressed(VirtualKeyCode::Minus) {
        canvas.dec_brush_size();
    }
    if let Some((x, y)) = input.mouse() {
        redraw_window |= canvas.mouse_at(input.mouse_held(0), [x, y]);
    }
    if input.key_pressed(VirtualKeyCode::Space) {
        canvas.color_wheel_toggle();
        if canvas.is_color_wheel_enabled() {
            window.set_cursor_icon(CursorIcon::Hand);
        } else {
            window.set_cursor_icon(CursorIcon::Default);
        }
        redraw_window = true;
    }
    redraw_window
}

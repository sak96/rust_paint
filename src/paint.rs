use crate::brush::{Brush, Point};
use crate::canvas::Canvas;
use crate::colorwheel::ColorWheel;
use rand::Rng;
use std::borrow::Cow;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BufferUsages, Color, CommandEncoderDescriptor, DeviceDescriptor, Features,
    FragmentState, Instance, Limits, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PowerPreference, PrimitiveState, PrimitiveTopology,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, VertexState,
};
use wgpu::{PushConstantRange, ShaderStages};

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
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
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &surface_config);

    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let push_constant = PushConstantRange {
        stages: ShaderStages::FRAGMENT,
        range: 0..std::mem::size_of::<ColorWheel>() as u32,
    };
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[push_constant],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("paint pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Point::desc()],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::LineList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
    });

    let mut brush = Brush::default();
    let mut canvas = Canvas::new(size);
    let mut colorwheel = ColorWheel::default();
    let mut strokes = vec![];
    let mut rng = rand::thread_rng();

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                surface_config.width = new_size.width;
                surface_config.height = new_size.height;
                surface.configure(&device, &surface_config);
                canvas.resize_window(new_size);
            }
            Event::RedrawRequested(_) => {
                let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&strokes),
                    usage: BufferUsages::VERTEX,
                });
                let output_texture = surface
                    .get_current_texture()
                    .expect("failed to get texture for rendering");
                let view = output_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("paint encoder"),
                });
                {
                    let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color::WHITE),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.set_push_constants(
                        ShaderStages::FRAGMENT,
                        0,
                        bytemuck::bytes_of(&colorwheel),
                    );
                    rpass.draw(0..strokes.len() as u32, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                output_texture.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                if input.update(&event) {
                    if input.key_pressed(VirtualKeyCode::Plus) {
                        brush.inc_radius()
                    }
                    if input.key_pressed(VirtualKeyCode::Minus) {
                        brush.dec_radius()
                    }
                    if input.key_pressed(VirtualKeyCode::Space) {
                        colorwheel.toggle();
                        window.request_redraw();
                    }
                    if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    if input.mouse_pressed(1) {
                        let color = [
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                        ];
                        brush.set_color(color);
                        colorwheel.set_color(color);
                        window.request_redraw();
                    }
                    if let Some((x, y)) = input.mouse() {
                        if let Some((start, end)) =
                            brush.draw_stroke(input.mouse_held(0), canvas.get_canvas_pos([x, y]))
                        {
                            strokes.push(start);
                            strokes.push(end);
                            window.request_redraw();
                        }
                    }
                }
            }
        }
    });
}

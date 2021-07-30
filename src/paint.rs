use std::borrow::Cow;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array, BackendBit, BufferUsage, Color, CommandEncoderDescriptor, DeviceDescriptor,
    Features, FragmentState, InputStepMode, Instance, Limits, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderFlags, ShaderModuleDescriptor, ShaderSource, SwapChainDescriptor,
    TextureUsage, VertexBufferLayout, VertexState,
};

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Point {
    pos: [f32; 2],
}

impl Point {
    fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Point>() as wgpu::BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x2],
        }
    }
}

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut input = WinitInputHelper::new();
    let mut size = window.inner_size();
    let instance = Instance::new(BackendBit::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(&ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        flags: ShaderFlags::all(),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Point::desc()],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[swapchain_format.into()],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::LineList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: MultisampleState::default(),
    });

    let mut sc_desc = SwapChainDescriptor {
        usage: TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
    let mut strokes = vec![];

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                sc_desc.width = new_size.width;
                sc_desc.height = new_size.height;
                size = new_size;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::RedrawRequested(_) => {
                let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&strokes),
                    usage: BufferUsage::VERTEX,
                });
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[RenderPassColorAttachment {
                            view: &frame.view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.draw(0..strokes.len() as u32, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                if input.update(&event) {
                    if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    let mouse_diff = input.mouse_diff();
                    if input.mouse_held(0) && (mouse_diff.0 != 0.0 || mouse_diff.1 != 0.0) {
                        if let Some((mouse_x, mouse_y)) = input.mouse() {
                            let width = size.width as f32;
                            let height = size.height as f32;
                            let mouse_x_wgpu = mouse_x / width - 0.5;
                            let mouse_y_wgpu = -mouse_y / height + 0.5;
                            let mouse_diff_wgpu = (mouse_diff.0 / width, mouse_diff.1 / (height));
                            strokes.push(Point {
                                pos: [mouse_x_wgpu, mouse_y_wgpu],
                            });
                            strokes.push(Point {
                                pos: [
                                    mouse_x_wgpu - mouse_diff_wgpu.0,
                                    mouse_y_wgpu + mouse_diff_wgpu.1,
                                ],
                            });
                            window.request_redraw();
                        }
                    }
                }
            }
        }
    });
}

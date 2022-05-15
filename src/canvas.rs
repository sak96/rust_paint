use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, BufferUsages, Color, CommandEncoderDescriptor, Device, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    PushConstantRange, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, Surface,
    SurfaceConfiguration, VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{brush::Point, colorwheel::ColorWheel};

pub struct Canvas {
    strokes: Vec<Point>,
    colorwheel: ColorWheel,
    surface: Surface,
    _adapter: Adapter,
    queue: Queue,
    device: Device,
    paint_pipeline: RenderPipeline,
    colorwheel_pipeline: RenderPipeline,
    surface_config: SurfaceConfiguration,
    colorwheel_enabled: bool,
}

impl Canvas {
    pub fn add_stroke(&mut self, start: Point, end: Point) {
        self.strokes.push(start);
        self.strokes.push(end);
    }

    pub fn color_wheel_toogle(&mut self) {
        self.colorwheel_enabled = !self.colorwheel_enabled;
    }

    #[must_use = "converted position must be used"]
    pub fn get_canvas_pos(&self, pos: [f32; 2]) -> [f32; 2] {
        self.colorwheel.get_canvas_pos(pos)
    }

    fn create_paint_pipeline(
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        let paint_shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("paint shader"),
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });
        let paint_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("paint layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("paint pipeline"),
            layout: Some(&paint_layout),
            vertex: VertexState {
                module: &paint_shader,
                entry_point: "vs_main",
                buffers: &[Point::desc()],
            },
            fragment: Some(FragmentState {
                module: &paint_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::LineList,
                ..PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_colorwheel_pipeline(
        device: &Device,
        surface_config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        let colorwheel_shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("color wheel shader"),
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("colorwheel.wgsl"))),
        });
        println!("{}",std::mem::size_of::<ColorWheel>() as u32);
        let colorwheel = PushConstantRange {

            stages: ShaderStages::FRAGMENT,
            range: 0..std::mem::size_of::<ColorWheel>() as u32,
        };
        let colorwheel_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("color wheel layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[colorwheel],
        });
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("color wheel pipeline"),
            layout: Some(&colorwheel_layout),
            vertex: VertexState {
                module: &colorwheel_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &colorwheel_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                ..PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }

    pub fn new(
        window_size: PhysicalSize<u32>,
        surface: Surface,
        device: Device,
        adapter: Adapter,
        queue: Queue,
    ) -> Self {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);
        let paint_pipeline = Self::create_paint_pipeline(&device, &surface_config);
        let colorwheel_pipeline = Self::create_colorwheel_pipeline(&device, &surface_config);
        Self {
            surface,
            surface_config,
            device,
            paint_pipeline,
            strokes: vec![],
            queue,
            colorwheel_enabled: false,
            colorwheel_pipeline,
            _adapter: adapter,
            colorwheel: ColorWheel::default(),
        }
    }

    pub fn redraw_canvas(&mut self) {
        let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.strokes),
            usage: BufferUsages::VERTEX,
        });
        let output_texture = self
            .surface
            .get_current_texture()
            .expect("failed to get texture for rendering");
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("paint encoder"),
            });
        {
            let view = output_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
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
            rpass.set_pipeline(&self.paint_pipeline);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw(0..self.strokes.len() as u32, 0..1);
            if self.colorwheel_enabled {
                rpass.set_pipeline(&self.colorwheel_pipeline);
                rpass.set_push_constants(
                    ShaderStages::FRAGMENT,
                    0,
                    bytemuck::bytes_of(&self.colorwheel),
                );
                rpass.draw(0..4, 0..1);
            }
        }
        self.queue.submit(Some(encoder.finish()));
        output_texture.present();
    }

    pub fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.colorwheel.set_size(new_size);
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.colorwheel.set_color(color);
    }
}

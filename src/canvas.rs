use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, BufferUsages, Color, CommandEncoderDescriptor, Device, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    PushConstantRange, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, Surface,
    SurfaceConfiguration, VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{
    brush::{Brush, Point},
    colorwheel::ColorWheel,
};

pub struct Canvas {
    strokes: Vec<Point>,
    colorwheel: ColorWheel,
    surface: Surface,
    _adapter: Adapter,
    queue: Queue,
    brush: Brush,
    device: Device,
    paint_pipeline: RenderPipeline,
    colorwheel_pipeline: RenderPipeline,
    surface_config: SurfaceConfiguration,
    colorwheel_enabled: bool,
    buffer_dimensions: PhysicalSize<u32>,
}

impl Canvas {
    pub fn brush_move(&mut self, brush_down: bool, new_pos: [f32; 2]) -> bool {
        if let Some((start, end)) = self
            .brush
            .draw_stroke(brush_down, self.colorwheel.get_canvas_pos(new_pos))
        {
            self.strokes.push(start);
            self.strokes.push(end);
            true
        } else {
            false
        }
    }

    pub fn color_wheel_toogle(&mut self) {
        self.colorwheel_enabled = !self.colorwheel_enabled;
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let buffer_dimensions = window_size;
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
            buffer_dimensions,
            colorwheel: ColorWheel::default(),
            brush: Brush::default(),
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

        let texture_extent = wgpu::Extent3d {
            width: self.buffer_dimensions.width,
            height: self.buffer_dimensions.height,
            depth_or_array_layers: 1,
        };
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("output buffer"),
            size: self.padded_bytes_per_row() * u64::from(self.buffer_dimensions.height),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &output_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        std::num::NonZeroU32::new(self.padded_bytes_per_row() as u32).unwrap(),
                    ),
                    rows_per_image: None,
                },
            },
            texture_extent,
        );
        self.queue.submit(Some(encoder.finish()));
        output_texture.present();
    }

    pub fn inc_brush_size(&mut self) {
        self.brush.inc_radius();
    }

    pub fn dec_brush_size(&mut self) {
        self.brush.dec_radius();
    }

    const fn padded_bytes_per_row(&self) -> u64 {
        let bytes_per_pixel = std::mem::size_of::<u32>();
        let unpadded_bytes_per_row = self.buffer_dimensions.width as usize * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        padded_bytes_per_row as u64
    }

    pub fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.buffer_dimensions = new_size;
        self.colorwheel.set_size(new_size);
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.brush.set_color(color);
        self.colorwheel.set_color(color);
    }
}

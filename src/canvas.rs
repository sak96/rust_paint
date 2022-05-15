use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, ShaderStages,
};
use winit::dpi::PhysicalSize;

use crate::{brush::Point, colorwheel::ColorWheel};

pub struct Canvas {
    window_size: PhysicalSize<u32>,
    strokes: Vec<Point>,
    colorwheel: ColorWheel,
}

impl Canvas {
    pub fn add_stroke(&mut self, start: Point, end: Point) {
        self.strokes.push(start);
        self.strokes.push(end);
    }

    #[must_use = "converted position must be used"]
    pub fn get_canvas_pos(&self, pos: [f32; 2]) -> [f32; 2] {
        let width = self.window_size.width as f32;
        let height = self.window_size.height as f32;
        [pos[0] / width - 0.5, -pos[1] / height + 0.5]
    }

    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            window_size,
            strokes: vec![],
            colorwheel: ColorWheel::default(),
        }
    }

    pub fn redraw_canvas(
        &mut self,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        render_pipeline: &wgpu::RenderPipeline,
        queue: &wgpu::Queue,
    ) {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.strokes),
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
            rpass.set_pipeline(render_pipeline);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.set_push_constants(
                ShaderStages::FRAGMENT,
                0,
                bytemuck::bytes_of(&self.colorwheel),
            );
            rpass.draw(0..self.strokes.len() as u32, 0..1);
        }
        queue.submit(Some(encoder.finish()));
        output_texture.present();
    }

    pub fn color_wheel_toogle(&mut self) {
        self.colorwheel.toggle();
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.colorwheel.set_color(color);
    }

    pub fn resize_window(&mut self, window_size: PhysicalSize<u32>) {
        self.window_size = window_size
    }
}

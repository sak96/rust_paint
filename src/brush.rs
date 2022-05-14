use wgpu::{vertex_attr_array, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[derive(Default)]
pub struct Brush {
    color: [f32; 3],
    pos: [f32; 2],
    down: bool,
    radius: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    color: [f32; 3],
    pos: [f32; 2],
    radius: f32,
}

impl Point {
    const ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3,  1 => Float32x2, 2 => Float32
    ];
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Point>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Brush {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color
    }

    pub fn inc_radius(&mut self) {
        if self.radius < 50.0 {
            self.radius += 0.5
        }
    }

    pub fn dec_radius(&mut self) {
        if self.radius > 0.5 {
            self.radius -= 0.5
        }
    }

    #[must_use = "stroke output must be used"]
    pub fn draw_stroke(&mut self, down: bool, pos: [f32; 2]) -> Option<(Point, Point)> {
        let prev_pos = self.pos;
        let prev_down = self.down;
        let radius = self.radius;
        self.down = down;
        self.pos = pos;
        if prev_down && pos != prev_pos {
            let color = self.color;
            Some((
                Point {
                    pos: prev_pos,
                    color,
                    radius,
                },
                Point { pos, color, radius },
            ))
        } else {
            None
        }
    }
}

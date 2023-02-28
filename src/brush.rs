use wgpu::{vertex_attr_array, VertexAttribute, VertexBufferLayout, VertexStepMode};


pub struct Brush {
    color: [f32; 3],
    pos: [f32; 2],
    down: bool,
    radius: f32,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            color: Default::default(),
            pos: Default::default(),
            down: Default::default(),
            radius: Self::BRUSH_MIN * 5.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    color: [f32; 3],
    pos: [f32; 2],
}

impl Point {
    const ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3,  1 => Float32x2, 2 => Float32
    ];

    #[must_use] pub const fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Brush {
    const BRUSH_MIN: f32 = 0.0001;
    const BRUSH_STEP: f32 = 0.0001;
    const BRUSH_MAX: f32 = 0.1;

    pub fn dec_radius(&mut self) {
        if self.radius > Self::BRUSH_MIN + Self::BRUSH_STEP {
            self.radius -= Self::BRUSH_STEP;
        }
    }

    #[must_use = "stroke output must be used"]
    pub fn draw_stroke(&mut self, down: bool, pos: [f32; 2]) -> Option<[Point; 6]> {
        let prev_pos = self.pos;
        let prev_down = self.down;
        self.down = down;
        self.pos = pos;
        if prev_down && pos != prev_pos {
            let color = self.color;
            let angle = f32::atan2(prev_pos[1] - pos[1], prev_pos[0] - pos[0]);
            let dx = f32::sin(angle) * self.radius / 2.0;
            let dy = f32::cos(angle) * self.radius / 2.0;
            Some([
                Point {
                    pos: [prev_pos[0] - dx, prev_pos[1] + dy],
                    color,
                },
                Point {
                    pos: [prev_pos[0] + dx, prev_pos[1] - dy],
                    color,
                },
                Point {
                    pos: [pos[0] + dx, pos[1] - dy],
                    color,
                },
                Point {
                    pos: [pos[0] + dx, pos[1] - dy],
                    color,
                },
                Point {
                    pos: [pos[0] - dx, pos[1] + dy],
                    color,
                },
                Point {
                    pos: [prev_pos[0] - dx, prev_pos[1] + dy],
                    color,
                },
            ])
        } else {
            None
        }
    }

    pub fn inc_radius(&mut self) {
        if self.radius < Self::BRUSH_MAX - Self::BRUSH_STEP {
            self.radius += Self::BRUSH_STEP;
        }
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
}

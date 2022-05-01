use wgpu::{vertex_attr_array, InputStepMode, VertexAttribute, VertexBufferLayout};

#[derive(Default)]
pub struct Brush {
    color: [f32; 3],
    pos: [f32; 2],
    down: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    color: [f32; 3],
    pos: [f32; 2],
}

impl Point {
    const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,  1 => Float32x2
    ];
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Point>() as wgpu::BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Brush {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color
    }

    #[must_use="stroke output must be used"]
    pub fn draw_stroke(&mut self, down: bool, pos: [f32; 2]) -> Option<(Point, Point)> {
        let prev_pos = self.pos;
        let prev_down = self.down;
        self.down = down;
        self.pos = pos;
        if prev_down && pos != prev_pos {
            let color = self.color;
            Some((
                Point {
                    pos: prev_pos,
                    color,
                },
                Point { pos, color },
            ))
        } else {
            None
        }
    }
}

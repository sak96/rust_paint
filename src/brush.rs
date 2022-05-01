#[derive(Default)]
pub struct Brush {
    color: [f32; 3],
}

impl Brush {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color
    }

    #[inline]
    pub fn color(&self) -> [f32; 3] {
        self.color
    }
}

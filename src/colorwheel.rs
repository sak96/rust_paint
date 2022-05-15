#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorWheel {
    color: [f32; 3],
    enabled: f32,
}

impl ColorWheel {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    pub fn toggle(&mut self) {
        self.enabled = 1.0 - self.enabled;
    }
}

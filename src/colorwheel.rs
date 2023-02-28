use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorWheel {
    // NOTE: alpha is never used.
    color: [f32; 4],
    size: [f32; 3],
}

impl ColorWheel {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = [color[0], color[1], color[2], 1.0];
    }

    #[must_use] pub fn get_canvas_pos(&self, pos: [f32; 2]) -> [f32; 2] {
        [pos[0] / self.size[0] - 0.5, -pos[1] / self.size[1] + 0.5]
    }
    pub fn set_size(&mut self, size: PhysicalSize<u32>){
        self.size[0] = size.width as f32;
        self.size[1] = size.height as f32;
    }
}

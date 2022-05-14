use winit::dpi::PhysicalSize;

pub struct Canvas {
    window_size: PhysicalSize<u32>,
}

impl Canvas {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self { window_size }
    }

    pub fn resize_window(&mut self, window_size: PhysicalSize<u32>) {
        self.window_size = window_size
    }

    #[must_use = "converted position must be used"]
    pub fn get_canvas_pos(&self, pos: [f32; 2]) -> [f32; 2] {
        let width = self.window_size.width as f32;
        let height = self.window_size.height as f32;
        [pos[0] / width - 0.5, -pos[1] / height + 0.5]
    }
}

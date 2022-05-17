use rand::Rng;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{canvas::Canvas};
pub struct InputHandler {
    rng: rand::prelude::ThreadRng,
}

impl InputHandler {
    pub fn handle_input(&mut self, input: &WinitInputHelper, canvas: &mut Canvas) -> bool {
        let mut redraw_window = false;
        if input.key_pressed(VirtualKeyCode::Plus) {
            canvas.inc_brush_size();
        }
        if input.key_pressed(VirtualKeyCode::Minus) {
            canvas.dec_brush_size();
        }
        if input.key_pressed(VirtualKeyCode::Space) {
            canvas.color_wheel_toogle();
            redraw_window = true;
        }
        if input.mouse_pressed(1) {
            let color = [
                self.rng.gen_range(0.0..1.0),
                self.rng.gen_range(0.0..1.0),
                self.rng.gen_range(0.0..1.0),
            ];
            canvas.set_color(color);
            redraw_window = true;
        }
        if let Some((x, y)) = input.mouse() {
            redraw_window |= canvas.brush_move(input.mouse_held(0), [x, y]);
        }
        redraw_window
    }

    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

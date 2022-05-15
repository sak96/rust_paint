use rand::Rng;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{brush::Brush, canvas::Canvas};
pub struct InputHandler {
    rng: rand::prelude::ThreadRng,
    brush: Brush,
}

impl InputHandler {
    pub fn handle_input(&mut self, input: &WinitInputHelper, canvas: &mut Canvas) -> bool {
        let mut redraw_window = false;
        if input.key_pressed(VirtualKeyCode::Plus) {
            self.brush.inc_radius();
        }
        if input.key_pressed(VirtualKeyCode::Minus) {
            self.brush.dec_radius();
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
            self.brush.set_color(color);
            canvas.set_color(color);
            redraw_window = true;
        }
        if let Some((x, y)) = input.mouse() {
            if let Some((start, end)) =
                self.brush.draw_stroke(input.mouse_held(0), canvas.get_canvas_pos([x, y]))
            {
                canvas.add_stroke(start, end);
                redraw_window = true;
            }
        }
        redraw_window
    }

    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            brush: Brush::default(),
        }
    }
}

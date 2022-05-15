#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::float_cmp)]

use winit::{event_loop::EventLoop, window::Window};

mod brush;
mod canvas;
mod colorwheel;
mod paint;
mod event;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    paint::run(event_loop, window);
}

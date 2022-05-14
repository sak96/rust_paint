use winit::{event_loop::EventLoop, window::Window};

mod brush;
mod canvas;
mod colorwheel;
mod paint;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    paint::run(event_loop, window);
}

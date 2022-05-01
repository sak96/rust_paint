use winit::{event_loop::EventLoop, window::Window};

mod brush;
mod paint;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    futures::executor::block_on(paint::run(event_loop, window));
}

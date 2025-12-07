mod shader;
mod winit_window;
mod chapters;
mod glfw;
mod shared;
mod extension;

// fn main2() {
//     let event_loop = EventLoop::new().unwrap();
//     event_loop.set_control_flow(ControlFlow::Wait);
    
//     let mut app = App::new();
//     event_loop.run_app(&mut app).unwrap();
// }

fn main() {
    // winit_window::run::<Renderer1>();
    chapters::texture::run();
}

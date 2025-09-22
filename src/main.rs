use gl::Viewport;
use glfw::{Action, Context, GlfwReceiver, Key, OpenGlProfileHint, WindowHint};

const WIDTH: u64 = 800;
const HEIGHT: u64 = 800;

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(glfw::fail_on_errors).expect("unable to initialize glfw");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    println!("Opengl version: {}", glfw::get_version_string());

    let (mut window, events) = glfw
        .create_window(800, 600, "title", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol).unwrap() as *const _);

    unsafe {
        Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
    }

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(1.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
        // println!("render {:.?}", std::time::SystemTime::now());
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            event => {
                println!("Unhandled event: {event:.?}")
            }
        }
    }
}

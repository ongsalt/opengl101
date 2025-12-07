use std::{ffi::CStr, str::Utf8Error};

use gl::types::GLenum;
use glfw::{Context, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowHint};
use image::GenericImageView;

pub fn get_string(value: GLenum) -> Result<String, Utf8Error> {
    unsafe {
        let raw_ptr = gl::GetString(value);
        CStr::from_ptr(raw_ptr as _)
            .to_str()
            .map(|it| it.to_string())
    }
}

pub fn load_image_from_file(path: &str) -> (Vec<u8>, u32, u32) {
    let img = image::open(path).expect("Failed to load image");

    // 1. Flip it vertically (OpenGL convention)
    let img = img.flipv();

    // 2. Convert to raw RGBA bytes (flat array)
    // This is the "Bitmap" in RAM
    let data = img.to_rgba8().into_raw();
    let (width, height) = img.dimensions();

    (data, width, height)
}

pub fn setup_glfw() -> (Glfw, PWindow, GlfwReceiver<(f64, glfw::WindowEvent)>) {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("unable to initialize glfw");

    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "title", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol).unwrap() as *const _);

    println!("- OpenGL version: {}", get_string(gl::VERSION).unwrap());
    println!("- Renderer: {}", get_string(gl::RENDERER).unwrap());

    (glfw, window, events)
}

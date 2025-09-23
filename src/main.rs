use core::str;
use std::{
    ffi::{CString, c_void},
    ptr,
};

use gl::{
    Viewport,
    types::{GLchar, GLint, GLsizeiptr},
};
use glfw::{Action, Context, GlfwReceiver, Key, OpenGlProfileHint, WindowHint};

// https://learnopengl.com/Getting-started/Hello-Triangle

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/1.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/1.frag");

fn main() {
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

    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_string = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_string.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_string = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_string.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // let mut info_log = Vec::with_capacity(512);
        // gl::GetProgramInfoLog(
        //     shader_program,
        //     512,
        //     ptr::null_mut(),
        //     info_log.as_mut_ptr() as *mut GLchar,
        // );
        // println!("ProgramInfoLog\n{}", str::from_utf8(&info_log).unwrap());

        gl::DeleteShader(fragment_shader);
        gl::DeleteShader(vertex_shader);

        // let mut success: GLint = 0;
        // gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        // println!("success = {success}");
        shader_program
    };

    let (vao, vbo) = unsafe {
        #[rustfmt::skip]
        let vertices: [f32; 9] = [
            0.0,  0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
        ];

        let mut vbo = 0;
        let mut vao = 0;

        gl::GenBuffers(1, &mut vbo);
        // println!("vbo: {vbo}");

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as _,
            &vertices[0] as *const _ as _, // fuck
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as i32,
            // &0 as *const _ as _,
            ptr::null()
        );

        gl::EnableVertexAttribArray(0);
        (vao, vbo)
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.4, 0.4, 0.4, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

use std::ptr;

use glfw::{Action, Context, GlfwReceiver, Key, OpenGlProfileHint, WindowHint};

use crate::{glfw::process_events, shader::Shader, shared::get_string};

// https://learnopengl.com/Getting-started/Hello-Triangle

// const VERTEX_SHADER_SOURCE: &str = include_str!("../../shaders/1.vert");
// const FRAGMENT_SHADER_SOURCE: &str = include_str!("../../shaders/1.frag");

pub fn run() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("unable to initialize glfw");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "title", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol).unwrap() as *const _);

    println!("OpenGL version: {}", glfw::get_version_string());
    println!("Renderer: {}", get_string(gl::RENDERER).unwrap());

    let shader = Shader::from_files("./shaders/1.vert", "./shaders/1.frag");

    let (vao, _, _) = unsafe {
        #[rustfmt::skip]
        let vertices: [f32; 18] = [
             0.0,  0.5,  0.0,  1.0,  0.0,  0.0,
             0.5, -0.5,  0.0,  0.0,  1.0,  0.0,
            -0.5, -0.5,  0.0,  0.0,  0.0,  1.0,
        ];

        // let indices: [u32; 6] = [0, 1, 2, 1, 2, 3];

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER,
        //     (indices.len() * size_of::<u32>()) as _,
        //     &indices[0] as *const _ as _, // fuck
        //     gl::STATIC_DRAW,
        // );

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as _,
            &vertices[0] as *const _ as _, // fuck
            gl::STATIC_DRAW,
        );

        // position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as i32,
            // &0 as *const _ as _,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (vao, vbo, ebo)
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // gl::UseProgram(shader_program);
            shader.use_program();
            gl::BindVertexArray(vao);

            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
        // println!("render {:.?}", std::time::SystemTime::now());
    }
}

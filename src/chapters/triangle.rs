use std::ptr;

use glfw::{Action, Context, GlfwReceiver, Key, OpenGlProfileHint, WindowHint};

use crate::{glfw::process_events, shader::Shader, shared::get_string};

// https://learnopengl.com/Getting-started/Hello-Triangle

pub fn run() {
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

    println!("OpenGL version: {}", get_string(gl::VERSION).unwrap());
    println!("Renderer: {}", get_string(gl::RENDERER).unwrap());

    let shader = Shader::from_files("./shaders/1.vert", "./shaders/1.frag");

    let (vao, _, ebo) = unsafe {
        #[rustfmt::skip]
        let vertices: [f32; 24] = [
             0.0,  0.5,  0.0,  1.0,  0.0,  0.0,
             0.5, -0.5,  0.0,  0.0,  1.0,  0.0,
            -0.5, -0.5,  0.0,  0.0,  0.0,  1.0,
             0.0, -1.0,  0.0,  1.0,  1.0,  1.0,
        ];

        let indices: [u32; 6] = [0, 1, 2, 1, 2, 3];

        let mut vbo = 0;
        gl::CreateBuffers(1, &mut vbo);
        gl::NamedBufferData(
            vbo,
            (vertices.len() * size_of::<f32>()) as _,
            &vertices[0] as *const _ as _, // fuck
            gl::STATIC_DRAW,
        );

        let mut vao = 0;
        gl::CreateVertexArrays(1, &mut vao);

        let mut ebo = 0;
        gl::CreateBuffers(1, &mut ebo);
        gl::NamedBufferData(
            ebo,
            (indices.len() * size_of::<u32>()) as _,
            &indices[0] as *const _ as _, // fuck
            gl::STATIC_DRAW,
        );

        gl::VertexArrayElementBuffer(vao, ebo);

        // position
        gl::VertexArrayAttribFormat(vao, 0, 3, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribBinding(vao, 0, 0);
        gl::EnableVertexArrayAttrib(vao, 0);

        // color
        gl::VertexArrayAttribFormat(vao, 1, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as u32);
        gl::VertexArrayAttribBinding(vao, 1, 0);
        gl::EnableVertexArrayAttrib(vao, 1);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, (6 * size_of::<f32>()) as _);

        (vao, vbo, ebo)
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            shader.use_program();

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
        // println!("render {:.?}", std::time::SystemTime::now());
    }
}

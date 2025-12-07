use std::ptr;

use cgmath::vec3;
use glfw::Context;

use crate::{
    glfw::process_events,
    shader::Shader,
    shared::{load_image_from_file, setup_glfw},
};

// https://learnopengl.com/Getting-started/Hello-Triangle

pub fn run() {
    let (mut glfw, mut window, mut events) = setup_glfw();

    let shader: Shader = Shader::from_files("./shaders/2.vert", "./shaders/2.frag");
    let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

    #[rustfmt::skip]
    let vertices: [f32; 32] = [
        // positions          // colors           // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom let
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
    ];

    let texture = unsafe {
        println!("loading");
        let (img, w, h) = load_image_from_file("./resources/xp.jpg");
        println!("done");

        let mut t = 0;
        gl::CreateTextures(gl::TEXTURE_2D, 1, &mut t);

        gl::TextureStorage2D(t, 1, gl::RGBA8, w as _, h as _);

        gl::TextureSubImage2D(
            t,
            0,
            0,
            0,
            w as _,
            h as _,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &img[0] as *const u8 as _,
        );
        // gl::GenerateTextureMipmap(t);

        gl::TextureParameteri(t, gl::TEXTURE_WRAP_R, gl::REPEAT as _);
        gl::TextureParameteri(t, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
        gl::TextureParameteri(t, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        gl::TextureParameteri(t, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

        t
    };

    // raw buffer
    let vbo = unsafe {
        let mut vbo = 0;
        gl::CreateBuffers(1, &mut vbo);
        gl::NamedBufferStorage(
            vbo,
            (vertices.len() * size_of::<f32>()) as _,
            &vertices[0] as *const _ as _, // fuck
            0,
        );
        vbo
    };

    // index stuff
    let ebo = unsafe {
        let mut ebo = 0;
        gl::CreateBuffers(1, &mut ebo);
        gl::NamedBufferStorage(
            ebo,
            (indices.len() * size_of::<u32>()) as _,
            &indices[0] as *const _ as _, // fuck
            0,
        );
        ebo
    };

    // how to parse those buffer into vertex
    let vao = unsafe {
        let mut vao = 0;
        gl::CreateVertexArrays(1, &mut vao);
        gl::VertexArrayElementBuffer(vao, ebo);

        gl::VertexArrayAttribFormat(vao, 0, 3, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribBinding(vao, 0, 0);
        gl::EnableVertexArrayAttrib(vao, 0);

        gl::VertexArrayAttribFormat(
            vao,
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as u32,
        );
        gl::VertexArrayAttribBinding(vao, 1, 0);
        gl::EnableVertexArrayAttrib(vao, 1);

        gl::VertexArrayAttribFormat(
            vao,
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (6 * size_of::<f32>()) as u32,
        );
        gl::VertexArrayAttribBinding(vao, 2, 0);
        gl::EnableVertexArrayAttrib(vao, 2);

        gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, (8 * size_of::<f32>()) as i32);
        vao
    };

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        while !window.should_close() {
            // println!("Loop");
            process_events(&mut window, &events);

            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindTextureUnit(0, texture);
            shader.use_program();

            gl::DrawElements(
                gl::TRIANGLES,
                // indices.len() as _,
                6,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            window.swap_buffers();
            glfw.poll_events();
        }
        // println!("render {:.?}", std::time::SystemTime::now());
    }
}

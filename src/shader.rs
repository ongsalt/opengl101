use std::{ffi::CString, fs, ptr};

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn from_files(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_source = fs::read_to_string(vertex_path).expect("cannot open shader file");
        let fragment_source = fs::read_to_string(fragment_path).expect("cannot open shader file");
        Self::new(vertex_source.as_str(), fragment_source.as_str())
    }

    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        let shader_program = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &CString::new(vertex_source).unwrap().as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &CString::new(fragment_source).unwrap().as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader);

            shader_program
        };

        Self { id: shader_program }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set_i32(key, value as i32);
    }

    pub fn set_i32(&mut self, key: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, key.as_ptr() as _),
                value as i32,
            );
        }
    }

    pub fn set_f32(&mut self, key: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, key.as_ptr() as _),
                value as f32,
            );
        }
    }
}

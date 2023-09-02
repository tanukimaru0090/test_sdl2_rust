extern crate gl;
extern crate sdl2;

use crate::sdl2_common::*;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = std::ffi::CString::new(source).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        return shader;
    }
}
fn link_shader_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        return program;
    }
}

pub struct Sdl2WindowData {
    title: String,
    pos: Sdl2Vector2f<u32>,
    meta: Sdl2MetaData,
}
impl Sdl2WindowData {
    fn new() -> Option<Sdl2WindowData> {
        return Some(Sdl2WindowData {
            title: "".to_string(),
            pos: Sdl2Vector2f::new(0, 0).unwrap(),
            meta: Sdl2MetaData::new().unwrap(),
        });
    }
}
pub struct Sdl2Window {
    window_data: Sdl2WindowData,
}
struct Sdl2MetaData {
    sdl2_context: sdl2::Sdl,
}
impl Sdl2MetaData {
    fn new() -> Option<Sdl2MetaData> {
        return Some(Sdl2MetaData {
            sdl2_context: sdl2::init().unwrap(),
        });
    }
}
impl Sdl2Window {
    pub fn new() -> Sdl2Window {
        return Sdl2Window {
            window_data: Sdl2WindowData::new().unwrap(),
        };
    }
    pub fn create_window(
        &mut self,
        title: &str,
        pos: Sdl2Vector2f<u32>,
    ) -> Result<&Sdl2Window, ()> {
        self.window_data.pos = pos;
        self.window_data.title = title.to_string().clone();
        return Ok(self);
    }
    pub fn show_window(&mut self) -> Result<&Sdl2Window, ()> {
        let video = self.window_data.meta.sdl2_context.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(2, 0);
        let window = video
            .window(
                &self.window_data.title,
                self.window_data.pos.x,
                self.window_data.pos.y,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("expect_error:window_expect_error!");
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        let vertex_shader_source = r#"
            #version 330 core     
            layout(location = 0) in vec3 postion ;
            void main() {
                gl_postion = vec4(postion,1.0);
            }
        "#;
        let fragment_shader_source = r#"
            #version 330 core 
            out vec4 color;
            void main() {
                color = vec4(1.0,0.0,0.0,1.0);
            }

        "#;
        let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
        let shader_program = link_shader_program(vertex_shader, fragment_shader);

        let mut event_pump = self.window_data.meta.sdl2_context.event_pump().expect("");
        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => {
                        return Ok(self);
                    }
                    _ => {}
                }
            }
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::UseProgram(shader_program);
            }
            {
                let ver: [f32; 12] = [
                    -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0,
                ];
                let colors: [f32; 12] =
                    [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
                let mut vbo: gl::types::GLuint = 0;
                unsafe {
                    gl::GenBuffers(1, &mut vbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (ver.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        ver.as_ptr() as *const gl::types::GLvoid,
                        gl::STATIC_DRAW,
                    );
                    let mut vao: gl::types::GLuint = 0;
                    gl::GenVertexArrays(1, &mut vao);
                    gl::BindVertexArray(vao);
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(
                        0,
                        3,
                        gl::FLOAT,
                        gl::FALSE,
                        (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                        std::ptr::null(),
                    );
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
            }
            window.gl_swap_window();
        }

        return Ok(self);
    }
}

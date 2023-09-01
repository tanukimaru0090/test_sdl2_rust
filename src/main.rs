extern crate gl;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
struct Sdl2Vector2f<T> {
    x: T,
    y: T,
}
impl<T> Sdl2Vector2f<T> {
    fn new(x: T, y: T) -> Option<Sdl2Vector2f<T>> {
        return Some(Sdl2Vector2f { x, y });
    }
}
#[derive(Copy, Clone)]
struct Sdl2Vector3f<T> {
    x: T,
    y: T,
    z: T,
}
impl<T> Sdl2Vector3f<T> {
    fn new(x: T, y: T, z: T) -> Option<Sdl2Vector3f<T>> {
        return Some(Sdl2Vector3f { x, y, z });
    }
}

struct Sdl2WindowData {
    title: String,
    pos: Sdl2Vector2f<u32>,
}
impl Sdl2WindowData {
    fn new() -> Option<Sdl2WindowData> {
        return Some(Sdl2WindowData {
            title: "".to_string(),
            pos: Sdl2Vector2f::new(0, 0).unwrap(),
        });
    }
}
struct Sdl2Window {
    window_data: Sdl2WindowData,
    //window_meta_data:Sdl2Window_Meta_Data,
}
struct Sdl2Window_Meta_Data {
    sdl_context: sdl2::Sdl,
}
impl Sdl2Window_Meta_Data {
    fn new() -> Option<Sdl2Window_Meta_Data> {
        return Some(Sdl2Window_Meta_Data {
            sdl_context: sdl2::init().unwrap(),
        });
    }
}
impl Sdl2Window {
    fn new() -> Sdl2Window {
        return Sdl2Window {
            window_data: Sdl2WindowData::new().unwrap(),
        };
    }
    fn create_window(&mut self, title: &str, pos: Sdl2Vector2f<u32>) -> Result<&Sdl2Window, ()> {
        self.window_data.pos = pos;
        self.window_data.title = title.to_string().clone();
        let sdl = Sdl2Window_Meta_Data::new();
        let mut sdl_context = sdl.unwrap().sdl_context;
        let video_subsystem = sdl_context.video().expect("");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(2, 0);
        let mut window = video_subsystem
            .window(title, pos.x, pos.y)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("expect_error:window_expect_error!");
        let mut gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        let mut event_pump = sdl_context.event_pump().expect("");
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
            }
            {
                let ver:[f32;12] = [
                    -0.5,-0.5,0.0,
                    0.5,-0.5,0.0,
                    0.5,0.5,0.0,
                    -0.5,0.5,0.0,
                ];
                let colors:[f32;12] = [
                    1.0,0.0,0.0,
                    0.0,1.0,0.0,
                    0.0,0.0,0.0,
                    1.0,1.0,1.0
                ];
                let mut vbo:gl::types::GLuint = 0;
                unsafe{
                    gl::GenBuffers(1,&mut vbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
                    gl::BufferData(gl::ARRAY_BUFFER,
                                    (ver.len()* std::mem::size_of::<f32>()) as
                                     gl::types::GLsizeiptr,
                                     ver.as_ptr() as *const gl::types::GLvoid,
                                    gl::STATIC_DRAW,
                    );
                    let mut vao:gl::types::GLuint = 0;
                    gl::GenVertexArrays(1,&mut vao);
                    gl::BindVertexArray(vao);
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(
                            0,
                            3,
                            gl::FLOAT,
                            gl::FALSE,
                            (3*std::mem::size_of::<f32>()) as gl::types::GLint,
                            std::ptr::null(),
                    );
                    gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
            }
            }
            window.gl_swap_window();
        }

        return Ok(self);
    }
    fn show_window(&self) -> Result<&Sdl2Window, ()> {
        return Ok(self);
    }
}
fn main() -> Result<(), String> {
    let sdl_window =
        Sdl2Window::new().create_window("てっすとー", Sdl2Vector2f::new(1280, 800).unwrap());
    return Ok(());
}

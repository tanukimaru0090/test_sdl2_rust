mod sdl2_common;
mod sdl2_window;
use sdl2_common::*;
use sdl2_window::*;
//Main Function
fn main() -> Result<(), String> {
    let mut sdl_window =Sdl2Window::new();
    sdl_window.create_window("てっすとー", Sdl2Vector2f::new(1280, 800).unwrap()).unwrap();
    let _res =  sdl_window.show_window();
    return Ok(());
}

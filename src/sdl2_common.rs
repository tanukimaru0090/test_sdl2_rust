#[derive(Copy, Clone)]
pub struct Sdl2Vector2f<T> {
    pub x: T,
    pub y: T,
}
impl<T> Sdl2Vector2f<T> {
    pub fn new(x: T, y: T) -> Option<Sdl2Vector2f<T>> {
        return Some(Sdl2Vector2f { x, y });
    }
}
#[derive(Copy, Clone)]
pub struct Sdl2Vector3f<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Sdl2Vector3f<T> {
    pub fn new(x: T, y: T, z: T) -> Option<Sdl2Vector3f<T>> {
        return Some(Sdl2Vector3f { x, y, z });
    }
}


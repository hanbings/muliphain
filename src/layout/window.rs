pub struct Window {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub buffer: *mut u32,
}

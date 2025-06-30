pub struct Desktop {
    pub id: u32,
    pub name: String,

    pub width: usize,
    pub height: usize,
    pub buffer: *mut u32,

    pub cursor_x: usize,
    pub cursor_y: usize,
    pub cursor_visible: bool,

    pub windows: Vec<crate::layout::window::Window>,
}
#![allow(unused)]

pub mod app;
pub mod compositor;
pub mod config;
pub mod graphic;
pub mod input;
pub mod layout;
pub mod security;

pub struct State {
    pub width: usize,
    pub height: usize,
    pub buffer: *mut u32
}

pub fn initialize(width: usize, height: usize, buffer: *mut u32) -> State {
    let config = config::initialize();
    let graphic = graphic::initialize();
    let compositor = compositor::initialize();
    let layout = layout::initialize(width, height, buffer);
    
    // Initialize application state
    let input = input::initialize();
    let security = security::initialize();
    let app = app::initialize();

    State {
        width,
        height,
        buffer,
    }
}
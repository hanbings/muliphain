#![allow(unused)]

use crate::{
    input::{
        keyboard::{self, Keyboard},
        mouse::Mouse,
    },
    layout::window::Window,
};

pub mod app;
pub mod compositor;
pub mod config;
pub mod graphic;
pub mod input;
pub mod layout;
pub mod security;

pub struct Muliphain {
    pub width: usize,
    pub height: usize,
    pub buffer: *mut u32,
    pub windows: Vec<Window>,
    pub mouse_radius: i32,
    pub dragging: bool,
    pub drag_window_id: Option<u32>,
    pub drag_offset: (i32, i32),
}

pub fn initialize(width: usize, height: usize, windows: Vec<Window>) -> Muliphain {
    let buffer = unsafe {
        std::alloc::alloc(
            std::alloc::Layout::from_size_align(
                (width * height) as usize * std::mem::size_of::<u32>(),
                std::mem::align_of::<u32>(),
            )
            .unwrap(),
        ) as *mut u32
    };

    Muliphain {
        width,
        height,
        buffer,
        windows,
        mouse_radius: 8,
        dragging: false,
        drag_window_id: None,
        drag_offset: (0, 0),
    }
}

impl Muliphain {
    pub fn render(&mut self, mouse: Mouse) {
        for window in &self.windows {
            let window_buffer = unsafe {
                std::slice::from_raw_parts_mut(
                    window.buffer,
                    (window.width * window.height) as usize,
                )
            };

            for y in 0..window.height {
                for x in 0..window.width {
                    let buffer_x = (window.x.wrapping_add(x)) as usize;
                    let buffer_y = (window.y.wrapping_add(y)) as usize;

                    if buffer_x < self.width && buffer_y < self.height {
                        let idx = buffer_y * self.width + buffer_x;
                        unsafe {
                            *self.buffer.add(idx) = window_buffer[(y * window.width + x) as usize];
                        }
                    }
                }
            }
        }

        let mut frame =
            unsafe { std::slice::from_raw_parts_mut(self.buffer, self.width * self.height) };
        self.draw_cursor(&mut frame, mouse.x, mouse.y);
    }

    fn draw_cursor(&self, buffer: &mut [u32], mx: i32, my: i32) {
        for dy in -self.mouse_radius..=self.mouse_radius {
            for dx in -self.mouse_radius..=self.mouse_radius {
                let x = mx + dx;
                let y = my + dy;

                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();
                    if distance <= self.mouse_radius as f32 {
                        let idx = y as usize * self.width + x as usize;
                        if distance > (self.mouse_radius as f32 - 2.0) {
                            buffer[idx] = 0xff000000;
                        } else {
                            buffer[idx] = 0xffffffff;
                        }
                    }
                }
            }
        }
    }

    pub fn handle_mouse(&mut self, mouse: Mouse) {
        if mouse.button == 1 {
            if !self.dragging {
                for i in (0..self.windows.len()).rev() {
                    let window = &self.windows[i];
                    if mouse.x >= window.x
                        && mouse.x < window.x + window.width
                        && mouse.y >= window.y
                        && mouse.y < window.y + 30
                    {
                        self.dragging = true;
                        self.drag_window_id = Some(window.id);
                        self.drag_offset = (mouse.x - window.x, mouse.y - window.y);

                        let window = self.windows.remove(i);
                        self.windows.push(window);
                        break;
                    }
                }
            } else if let Some(id) = self.drag_window_id {
                if let Some(window) = self.windows.iter_mut().find(|w| w.id == id) {
                    window.x = mouse.x - self.drag_offset.0;
                    window.y = mouse.y - self.drag_offset.1;
                }
            }
        } else {
            self.dragging = false;
            self.drag_window_id = None;
        }
    }

    pub fn add_window(&mut self, x: i32, y: i32, width: i32, height: i32) -> *mut u32 {
        let buffer = unsafe {
            std::alloc::alloc(
                std::alloc::Layout::from_size_align(
                    (width * height) as usize * std::mem::size_of::<u32>(),
                    std::mem::align_of::<u32>(),
                )
                .unwrap(),
            )
        } as *mut u32;

        let window = Window {
            id: self.windows.len() as u32,
            width,
            height,
            x,
            y,
            buffer,
        };

        self.windows.push(window);

        buffer
    }
}

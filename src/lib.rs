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

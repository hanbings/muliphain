pub mod device;
pub mod renderer;

pub(crate) fn alloc_buffer(size: usize) -> *mut u32 {
    0 as *mut u32
}

pub(crate) fn free_buffer(_buffer: *mut u32) {
}

pub(crate) fn draw_direcr(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u32,
) {
    if x >= width || y >= height {
        return;
    }
    let x_end = (x + w).min(width);
    let y_end = (y + h).min(height);

    for j in y..y_end {
        for i in x..x_end {
            buffer[j * width + i] = color;
        }
    }
}

pub fn initialize() {}
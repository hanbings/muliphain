pub mod device;
pub mod renderer;

pub(crate) fn alloc_buffer(size: usize) -> *mut u32 {
    0 as *mut u32
}

pub(crate) fn free_buffer(_buffer: *mut u32) {}

pub fn initialize() {}

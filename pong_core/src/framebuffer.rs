use alloc::vec::Vec;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            width,
            height,
            buffer: Vec::with_capacity(width*height)
        }
    }

    pub fn set_pixel(&mut self, value: u32, x: usize, y: usize) {
        self.buffer[y*self.width + x] = value
    }
}
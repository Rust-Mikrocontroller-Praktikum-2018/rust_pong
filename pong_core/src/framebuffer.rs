use alloc::vec::Vec;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width*height);
        buffer.resize(width*height, 0);

        FrameBuffer {
            width,
            height,
            buffer: buffer
        }
    }

    pub fn set_pixel(&mut self, value: u32, x: usize, y: usize) {
        self.buffer[y*self.width + x] = value
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u32 {
        self.buffer[y*self.width + x]
    }
}
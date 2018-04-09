use framebuffer::FrameBuffer;

pub trait Display {
    fn set_pixel(&mut self, x: usize, y: usize, hex_color: u32);
}
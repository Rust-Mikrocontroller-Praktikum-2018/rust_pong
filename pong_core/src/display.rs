use framebuffer::FrameBuffer;

pub trait Display {
    fn set_pixel(&mut self, x: usize, y: usize, hex_color: u32);
    fn show_score(&mut self, score_1: usize, score_2: usize, hex_color: u32);
}
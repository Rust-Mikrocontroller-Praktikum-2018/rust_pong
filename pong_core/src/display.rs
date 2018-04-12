use framebuffer::FrameBuffer;

pub trait Display {
    fn set_pixel_1(&mut self, x: usize, y: usize, hex_color: u32);
    fn set_pixel_2(&mut self, x: usize, y: usize, hex_color: u32);
    fn unset_pixel_1(&mut self, x: usize, y: usize);
    fn unset_pixel_2(&mut self, x: usize, y: usize);
    fn show_score(&mut self, score_1: usize, score_2: usize, hex_color: u32);
}
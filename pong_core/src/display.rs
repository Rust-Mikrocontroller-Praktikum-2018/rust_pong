use framebuffer::FrameBuffer;

pub trait Display {
    fn show(&mut self, frame_buffer: &FrameBuffer);
}
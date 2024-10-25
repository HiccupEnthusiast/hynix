pub struct FramebufferHelper<'a> {
    inner: limine::framebuffer::Framebuffer<'a>,
}
impl<'a> FramebufferHelper<'a> {
    pub fn new(request: &'a limine::request::FramebufferRequest) -> Option<Self> {
        let inner = request.get_response()?.framebuffers().next()?;
        Some(Self { inner })
    }
    pub fn pitch(&self) -> u64 {
        self.inner.pitch()
    }
    pub unsafe fn put_pixel(&self, x: u64, y: u64, color: Color) {
        *(self
            .inner
            .addr()
            .add((x * self.inner.bpp() as u64 / 8 + y * self.pitch()) as usize)
            as *mut u32) = 0x00 << 24 | color.raw();
    }
}
#[repr(transparent)]
pub struct Color(pub [u8; 3]);
impl Color {
    pub fn raw(&self) -> u32 {
        (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32) << 0
    }
}

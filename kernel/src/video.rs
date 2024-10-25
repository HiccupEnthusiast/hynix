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
    pub unsafe fn put_pixel(&self, x: u64, y: u64) {
        *(self
            .inner
            .addr()
            .add((x * self.inner.bpp() as u64 / 8 + y * self.pitch()) as usize)
            as *mut u32) = 0xFFFF0000;
    }
}

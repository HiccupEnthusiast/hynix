const FONT_BYTES: &[u8] = include_bytes!("tcvn8x16.psf");

pub struct FramebufferHelper<'a> {
    inner: limine::framebuffer::Framebuffer<'a>,
}
impl<'a> FramebufferHelper<'a> {
    pub fn new(request: &'a limine::request::FramebufferRequest) -> Option<Self> {
        let inner = request.get_response()?.framebuffers().next()?;
        Some(Self { inner })
    }
    pub unsafe fn put_pixel(&self, x: u64, y: u64, color: Color) {
        *(self
            .inner
            .addr()
            .add((x * self.inner.bpp() as u64 / 8 + y * self.inner.pitch()) as usize)
            as *mut u32) = 0x00 << 24 | color.raw();
    }
    pub unsafe fn put_character(&self, ch_point: u8, cx: u64, cy: u64) {
        // The parsing of this file only works because tcvn is a 8 x 16 PSFv2 font with 256 glyphs with no unicode table
        // more fonts to be supported then proper parsing of headers has to be done.
        let offset = 32 + (ch_point as usize * 16);
        let character = &FONT_BYTES[offset..offset + 16];

        for (idx, byte) in character.iter().enumerate() {
            for shift in 0..8 {
                let pixel = if (byte >> shift) & 0b1 == 1 {
                    Color([0xff, 0xff, 0xff])
                } else {
                    Color([0x00, 0x00, 0x00])
                };
                self.put_pixel((7 - shift) + cx * 8, (idx as u64) + cy * 16, pixel);

                /*com_print!(
                    "{}",
                    if (byte >> (7 - shift)) & 0b1 == 1 {
                        "1"
                    } else {
                        " "
                    }
                );*/
            }
            //com_println!();
        }
    }
    pub unsafe fn put_string(&self, input: &[u8]) {
        for (idx, byte) in input.into_iter().enumerate() {
            //com_println!("{}: {}", *byte as char, idx as u64 / (self.pitch() / 8));
            self.put_character(*byte, idx as u64, idx as u64 / self.characters_per_row());
        }
    }

    pub fn characters_per_row(&self) -> u64 {
        // R = pitch / (bytes_per_pixel * size_of_character)
        // bytes_per_pixel = bpp / 8
        // size_of_character = 8
        // Since the font is 8 bytes wide then R can be simplifed to
        self.inner.pitch() / (self.inner.bpp() as u64)
    }
    pub fn pitch(&self) -> u64 {
        self.inner.pitch()
    }
}
#[repr(transparent)]
pub struct Color(pub [u8; 3]);
impl Color {
    pub fn raw(&self) -> u32 {
        (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32) << 0
    }
}

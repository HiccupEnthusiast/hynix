#![no_std]
#![no_main]

use core::arch::asm;

use limine::{
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
    BaseRevision,
};
use video::{Color, FramebufferHelper};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

mod serial;
mod video;

/// # Safety
///
/// This function works properly as long as:
/// a) It has #[no_mangle]
/// b) It accepts no arguments
/// c) The linker knows that _start is the entry of the program
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    match FramebufferHelper::new(&FRAMEBUFFER_REQUEST) {
        Some(framebuffer) => {
            com_println!("Bytes per row in the framebuffer: {}", framebuffer.pitch());
            for i in 0..100_u64 {
                framebuffer.put_pixel(i, i, Color([0xff, 0x00, 0xff]));
            }
        }
        None => com_println!("Couldn't get framebuffer"),
    };

    halt()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    unsafe { asm!("hlt") }
    loop {}
}

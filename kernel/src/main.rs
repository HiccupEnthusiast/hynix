#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;

use limine::{
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
    BaseRevision,
};
use serial::ComDebugger;

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

/// # Safety
///
/// This function works properly as long as:
/// a) It has #[no_mangle]
/// b) It accepts no arguments
/// c) The linker knows that kmain is the entry of the program
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            com_println!("Bytes per row in the framebuffer: {}", framebuffer.pitch());
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

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

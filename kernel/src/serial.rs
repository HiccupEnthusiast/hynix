use core::{arch::asm, fmt::Write};

#[macro_export]
macro_rules! com_print {
    ($($args:tt)*) => ({
        use ::core::fmt::Write;
        write!($crate::serial::ComDebugger::new(), $($args)*);
    });
}
#[macro_export]
macro_rules! com_println {
    () => {
        $crate::com_print!("\n")
    };
    ($s:expr) => {
        $crate::com_print!(concat!($s, "\n"))
    };
    ($s:expr, $($args:tt)+) => {
        $crate::com_print!(concat!($s, "\n"), $($args)*)
    };
}

pub const COM1_PORT: u16 = 0x3f8;

pub struct ComDebugger {
    _inner: (),
}
impl Write for ComDebugger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.outb(byte);
        }
        Ok(())
    }
}

impl ComDebugger {
    pub fn new() -> Self {
        // # Safety
        //
        // COM1_PORT is a serial address
        unsafe {
            init_com_port(COM1_PORT);
        }
        Self { _inner: () }
    }
    pub fn outb(&self, byte: u8) {
        // # Safety
        //
        // COM1_PORT is a serial address
        unsafe {
            outb(COM1_PORT, byte);
        }
    }
}

/// # Safety
///
/// Port should be a valid serial port address
pub unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("dx") port, in("al") data)
}
/// # Safety
///
/// Port should be a valid serial port address
pub unsafe fn inb(port: u16) -> u8 {
    let mut byte: u8;
    asm!("in al, dx", out("al") byte, in("dx") port);
    byte
}
/// # Safety
///
/// Port should be a valid serial port address
pub unsafe fn init_com_port(port: u16) {
    outb(port + 1, 0b00000000); // Disable all interrupts
    outb(port + 3, 0b10000000); // Enable DLAB
    outb(port + 0, 0b00000011); // (lo. bytes) Baud divisor
    outb(port + 1, 0b00000000); // (hi. bytes)
    outb(port + 3, 0b00000011); // 8 bytes, 1 stop bit, no parity, no break and clear DLAB
    outb(port + 2, 0b11000111); // Enable FIFO, clear after transactions, 14 bytes before interrupt

    outb(port + 4, 0b00011110); // RTS OUT#1 & OUT#2 set, loopback mode
    outb(port + 0, 0b10101010); // Test byte
    assert_eq!(inb(port), 0b10101010);

    // not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled
    outb(port + 4, 0b00001111);
}

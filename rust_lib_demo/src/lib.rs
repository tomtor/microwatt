#![no_std]
#![feature(alloc_error_handler)]

use core::fmt::Write;
use core::panic::PanicInfo;

use heapless::consts::*;
use heapless::String;

extern crate linked_list_allocator;
use linked_list_allocator::*;
#[global_allocator]
static mut HEAP: LockedHeap = LockedHeap::empty();

extern crate alloc;
use alloc::vec::Vec;

extern crate cty;

extern "C" {
    fn putchar(c: cty::c_char) -> ();
}

pub fn print(s: &str) {
    for c in s.bytes() {
        unsafe { putchar(c) };
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    print("Rust\r\n");

    const HEAP_SIZE: usize = 1024;
    static mut HEAP_AREA: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP = LockedHeap::new(&HEAP_AREA[0] as *const u8 as usize, HEAP_AREA.len()) };

    //let mut xs = Vec::new();
    for i in 2..=3 {
        let mut s: String<U128> = String::new();
        writeln!(s, "i {}\r", i).ok();
        // fp needs CRNOR instruction
        //writeln!(s, "{}\r", 1.0 / i as f32).ok();
        print(&s);
        // Also hits unimplemented instr:
        //xs.push(i);
    }
    panic!("test");
    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    unsafe {
        putchar('!' as u8);
    }
    let mut s: String<U128> = String::new();
    writeln!(s, "{}\r", panic_info).ok();
    print(&s);
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Heap");
}


// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffer::print_something();
    println!("Hello World{}", "!");

    blog_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
    // uncomment line below to trigger a stack overflow
    stack_overflow();
    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    // // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // as before
    #[cfg(test)]
    test_main();

    loop {
        use blog_os::print;
        print!("-"); // new
    }

    println!("It did not crash!");
    blog_os::hlt_loop();

    loop {}
}

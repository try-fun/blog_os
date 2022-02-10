// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {}
    blog_os::hlt_loop();
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

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // uncomment line below to trigger a stack overflow
    // stack_overflow();
    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    // // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe {
    //     *ptr = 42;
    // }

    // Note: The actual address might be different for you. Use the address that
    // your page fault handler reports.
    // let ptr = 0x20653f as *mut u32;

    // // read from a code page
    // unsafe {
    //     let x = *ptr;
    // }
    // println!("read worked");

    // // write to a code page
    // unsafe {
    //     *ptr = 42;
    // }
    // println!("write worked");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    println!("It did not crash!");

    // loop {
    //     // use blog_os::print;
    //     // for _ in 0..100000 {}
    //     // print!("-");
    // }
    blog_os::hlt_loop();
}

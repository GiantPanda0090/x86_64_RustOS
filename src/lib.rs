#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![feature(alloc)] // the alloc crate is still unstable
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(const_atomic_usize_new)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate volatile;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;
extern crate linked_list_allocator;

#[macro_use]
extern crate once;
#[macro_use]
extern crate alloc;

#[macro_use]
mod vga_buffer;
mod memory;

use memory::heap_allocator::BumpAllocator;
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();


#[no_mangle]
pub extern fn rust_main(multiboot_information_address:usize) {
// ATTENTION: we have a very small stack and no guard page
vga_buffer::clear_screen();//clean screen //KEEP

//main
//assign 2 task
//println!("{}", { println!("inner"); "outer" });
//println!("Hello World{}", "!");
//vga_buffer::print_something();

//panic test
//panic!();

//Welcome screen
let hello = b"Blog OS";
let color_byte = 0x1f; // white foreground, blue background

let mut hello_colored = [color_byte; 14];
for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i*2] = *char_byte;
}
// write to the center of the VGA text buffer
let buffer_ptr = (0xb8000 + 1988) as *mut _;
unsafe { *buffer_ptr = hello_colored };
//wait....
let mut count=0u32;
loop{
count+=1;

if count ==19990000{
break;
}
}

vga_buffer::clear_screen();//clean screen

//**********************************************************************************************************//


//print memory
let boot_info=unsafe{multiboot2::load(multiboot_information_address)};//KEEP

    enable_nxe_bit();//KEEP
    enable_write_protect_bit();//KEEP
       // set up guard page and map the heap pages REFACTORED!!
           memory::init(boot_info,multiboot_information_address);
           unsafe {
               HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
           }
//HEAP TEST
    use alloc::boxed::Box;
let mut heap_test = Box::new(42);
*heap_test -= 15;
let heap_test2 = Box::new("hello");
println!("{:?} {:?}", heap_test, heap_test2);

let mut vec_test = vec![1,2,3,4,5,6,7];
vec_test[3] = 42;
for i in &vec_test {
    print!("{} ", i);
}


println!("\nBoot complete without error!!");
let mut count=0u32;
loop{
count+=1;

if count ==100000000{
break;
}
}

let str = b"I am formating myself 1000000000000 times because i am stupid";
let color_str = 0x1f; // white foreground, blue background
let mut str_colored = [color_str; 122];
//MEMORY LEAK test
vga_buffer::clear_screen();//clean screen
for (i, char_byte) in str.into_iter().enumerate() {
    format!("Some String");
    str_colored[i*2] = *char_byte;
    // write to the center of the VGA text buffer
    let buffer_strptr = (0xb8000 + 1780) as *mut _;
    unsafe { *buffer_strptr = str_colored };
    let mut count=0u32;
    loop{
    count+=1;

    if count ==10000000{
    break;
    }
    //println!("Formating {}",i);

    }
}

//MEMORY LEAK test
//for i in 0..10000 {
//format!("Some String");
//}



//Fibonacci
//let z = fib(12);
//let(ans1,ans2)=z;
//println!("Fibonacci of ({}) is ({}) \n",12,ans1);

//fmt write
//use core::fmt::Write;

  //  vga_buffer::WRITER.lock().write_str("Hello again");
  //  write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);



  //loop
    loop{}

//not reachable
let hello = b"Hello World!";
let color_byte = 0x1f; // white foreground, blue background

let mut hello_colored = [color_byte; 24];
for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i*2] = *char_byte;
}

// write `Hello World!` to the center of the VGA text buffer
let buffer_ptr = (0xb8000 + 1988) as *mut _;
unsafe { *buffer_ptr = hello_colored };

loop{}
//****************************************************************
}

pub fn fib(n:u32)->(u32,u32){
if n==1{
(1,0)
}else{
println!("better({})",n-1);
let (f1,f2)=fib(n-1);
println!("current answer is ({})",f1);
(f1+f2,f1)
}
}

//enable nxe bit
fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
    println!("enable the NXE bit");
}
//enable writable bit
fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments,file: &'static str,line:u32) -> !
{
println!("\n\nPANIC in {} at line {}:",file, line);
println!("      {}",fmt);
loop{};
}

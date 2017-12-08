use multiboot2::BootInformation;
use memory::area_frame_allocator::AreaFrameAllocator;
pub use memory::paging::remap_the_kernel;
pub use memory::paging::test_paging;
use memory::paging::*;
use {HEAP_START, HEAP_SIZE};

pub fn init(boot_info: &BootInformation,multiboot_information_address:usize) {
        assert_has_not_been_called!("memory::init must be called only once");

    let memory_map_tag = boot_info.memory_map_tag().expect(
           "Memory map tag required");

     for area in memory_map_tag.memory_areas(){
           println!("start:0x{:x}, length: 0x{:x}",area.base_addr, area.length);
           }
       let elf_sections_tag = boot_info.elf_sections_tag().expect(
           "Elf sections tag required");
           println!("kernel sections:");
           for section in elf_sections_tag.sections() {
               println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                   section.addr, section.size, section.flags);
           }
       let kernel_start = elf_sections_tag.sections()
           .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap();
       let kernel_end = elf_sections_tag.sections()
           .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max()
           .unwrap();

       println!("kernel start: {:#x}, kernel end: {:#x}",
                kernel_start,
                kernel_end);
       println!("multiboot start: {:#x}, multiboot end: {:#x}",
                boot_info.start_address(),
                boot_info.end_address());
                //multiboot information structure
                let multiboot_start = multiboot_information_address;
                let multiboot_end = multiboot_start + (boot_info.total_size as usize);
                println!("multiboot start: 0x{:x}, multiboot end: 0x{:x},size is : {} bytes", multiboot_start,multiboot_end,multiboot_end-multiboot_start);
                //AreaFrameAllocator
                //re export
       let mut frame_allocator = AreaFrameAllocator::new(
           kernel_start as usize, kernel_end as usize,
           boot_info.start_address(), boot_info.end_address(),
           memory_map_tag.memory_areas());
            use memory::paging::*;

            //heap mapping
            let mut active_table = remap_the_kernel(&mut frame_allocator,
        boot_info);



    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, WRITABLE, &mut frame_allocator);


          test_paging(&mut frame_allocator);
           use memory::FrameAllocator;
           // initialize the heap allocator

           frame_allocator.allocate_frame(); // new: try to allocate a frame
}
println!("Kernal Mapping Complete!");

}

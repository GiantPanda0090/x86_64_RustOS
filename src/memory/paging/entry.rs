use memory::Frame; // needed later

pub struct Entry(u64);
use multiboot2::ElfSection;


//entry formate
    bitflags! {
        pub struct EntryFlags: u64 {
            const PRESENT =         1 << 0; //0,PRESENT
            const WRITABLE =        1 << 1; //1,writable
            const USER_ACCESSIBLE = 1 << 2; //2,  USER_ACCESSIBLE
            const WRITE_THROUGH =   1 << 3; //3, write though caching
            const NO_CACHE =        1 << 4; //4, disable cache
            const ACCESSED =        1 << 5; //5, ACCESSED
            const DIRTY =           1 << 6; //6, DIRTY
            const HUGE_PAGE =       1 << 7; //7,huge page or null
            const GLOBAL =          1 << 8; //8, global
            const NO_EXECUTE =      1 << 63;//63 no NO_EXECUTE
        }
    }
// page table entry
impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

//extract flag from entry
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }
// extract physical address
    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(PRESENT) {
            Some(Frame::containing_address(
                self.0 as usize & 0x000fffff_fffff000
            ))
        } else {
            None
        }
    }
//modify entries
    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert!(frame.start_address() & !0x000fffff_fffff000 == 0);
        self.0 = (frame.start_address() as u64) | flags.bits();
    }
}


impl EntryFlags {
    pub fn from_elf_section_flags(section: &ElfSection) -> EntryFlags {
        use multiboot2::{ELF_SECTION_ALLOCATED, ELF_SECTION_WRITABLE,
            ELF_SECTION_EXECUTABLE};

        let mut flags = EntryFlags::empty();

        if section.flags().contains(ELF_SECTION_ALLOCATED) {
            // section is loaded to memory
            flags = flags | PRESENT;
        }
        if section.flags().contains(ELF_SECTION_WRITABLE) {
            flags = flags | WRITABLE;
        }
        if !section.flags().contains(ELF_SECTION_EXECUTABLE) {
            flags = flags | NO_EXECUTE;
        }

        flags
    }
}

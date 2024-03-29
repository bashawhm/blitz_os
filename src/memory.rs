use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{PageTable, Mapper, Page, RecursivePageTable, Size4KiB, FrameAllocator, PhysFrame};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator<I> where I: Iterator<Item = PhysFrame> {
    frames: I,
}

impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I> where I: Iterator<Item = PhysFrame> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}

pub fn init_frame_allocator(memory_map: &'static MemoryMap) -> BootInfoFrameAllocator<impl Iterator<Item = PhysFrame>> {
    //Get usable regions
    let regions = memory_map.iter().filter(|r| r.region_type == MemoryRegionType::Usable);
    //Map each region to its address space
    let addr_ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());
    //transform to an iterator of frame start addresses
    let frame_address = addr_ranges.flat_map(|r| r.into_iter().step_by(4096));
    //Create `PhysFrame` types from start addressess
    let frames = frame_address.map(|addr| {
        PhysFrame::containing_address(PhysAddr::new(addr))
    });

    BootInfoFrameAllocator {frames}
}

pub unsafe fn init(level_4_table_addr: usize) -> RecursivePageTable<'static> {
    /// Rust currently treats the whole body of unsafe functions as an unsafe
    /// block, which makes it difficult to see which operations are unsafe. To
    /// limit the scope of unsafe we use a safe inner function.
    fn init_inner(level_4_table_addr: usize) -> RecursivePageTable<'static> {
        let level_4_table_ptr = level_4_table_addr as *mut PageTable;
        let level_4_table = unsafe { &mut *level_4_table_ptr };
        RecursivePageTable::new(level_4_table).unwrap()
    }

    init_inner(level_4_table_addr)
}

//Returns the physical address for the virtual address addr, or None if the virtual address is not mapped
pub fn translate_addr(addr: u64, recursive_page_table: &RecursivePageTable) -> Option<PhysAddr> {
    let addr = VirtAddr::new(addr);
    let page: Page = Page::containing_address(addr);

    //Perform the translation
    let frame = recursive_page_table.translate_page(page);
    frame.map(|frame| frame.start_address() + u64::from(addr.page_offset()))
}

pub fn create_example_mapping(recursive_page_table: &mut RecursivePageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let page: Page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        recursive_page_table.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

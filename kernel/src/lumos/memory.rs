use x86_64::{
    structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB}, PhysAddr, VirtAddr
};
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};

pub struct BootInfoFrameAllocator
{
    mem_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(mem_regions: &'static MemoryRegions) -> Self
    {
        Self
        {
            mem_regions,
            next: 0,
        }
    }

    /// This returns an iterator of USABLE frames only!
    fn frames(&self) -> impl Iterator<Item = PhysFrame>
    {
        let regions = self.mem_regions.iter();
        let available = regions.
            filter(|r| r.kind == MemoryRegionKind::Usable);
        
        let addr_ranges = available
            .map(|r| r.start..r.end);

        let addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(offset: VirtAddr) -> OffsetPageTable<'static>
{
    unsafe
    {
        let lv4 = enable_level_4_page_table(offset);
        OffsetPageTable::new(lv4, offset)
    }
}

unsafe fn enable_level_4_page_table(offset: VirtAddr) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (lv4, _) = Cr3::read();

    let phys = lv4.start_address();
    let virt = offset + phys.as_u64();

    let pt_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *pt_ptr }
}

pub fn create_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    fa: &mut impl FrameAllocator<Size4KiB>
)
{
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, fa)
    };

    result.expect("Failed during mapping!").flush();
}
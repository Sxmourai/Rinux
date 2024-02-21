use core::alloc::Layout;

#[global_allocator]
static mut ALLOCATOR: DummyAlloc = DummyAlloc{};

struct DummyAlloc {}

unsafe impl core::alloc::GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
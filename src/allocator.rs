use alloc::alloc::{GlobalAlloc, Layout};
use windows::Win32::{
    Foundation::HANDLE,
    System::Memory::{
        GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_NONE, HEAP_ZERO_MEMORY,
    },
};

unsafe fn process_heap_unchecked() -> HANDLE {
    GetProcessHeap().unwrap_unchecked()
}

pub struct WinAllocator;

unsafe impl GlobalAlloc for WinAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(process_heap_unchecked(), HEAP_NONE, layout.size()).cast()
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(process_heap_unchecked(), HEAP_ZERO_MEMORY, layout.size()).cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        HeapFree(process_heap_unchecked(), HEAP_NONE, Some(ptr.cast())).unwrap_or_default();
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        HeapReAlloc(
            process_heap_unchecked(),
            HEAP_NONE,
            Some(ptr.cast()),
            new_size,
        )
        .cast()
    }
}

#[global_allocator]
static ALLOCATOR: WinAllocator = WinAllocator;

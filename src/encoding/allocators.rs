use std::alloc::Layout;

use capnp::{message::Allocator, private::units::BYTES_PER_WORD};

#[repr(align(8))]
pub struct CapnpAlloc<const N: usize> {
    buf: [u8; N],
    called: bool,
}

unsafe impl<const N: usize> Allocator for CapnpAlloc<N> {
    #[inline]
    fn allocate_segment(&mut self, size_words: u32) -> (*mut u8, u32) {
        if self.called {
            panic!("CapnpAlloc::allocate_segment called multiple times");
        }

        let size = (size_words * 8) as usize;

        self.called = true;

        if size > N {
            panic!("Not enough space in CapnpAlloc");
        }

        (self.buf.as_mut_ptr(), (N / 8) as u32)
    }

    #[inline]
    unsafe fn deallocate_segment(&mut self, _ptr: *mut u8, _word_size: u32, _words_used: u32) {}
}

impl<const N: usize> CapnpAlloc<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            called: false,
        }
    }
}

impl<const N: usize> Default for CapnpAlloc<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CapnpBorrowAlloc<'a> {
    buf: &'a mut [u8],
    called: bool,
}

unsafe impl<'a> Allocator for CapnpBorrowAlloc<'a> {
    #[inline]
    fn allocate_segment(&mut self, size_words: u32) -> (*mut u8, u32) {
        if self.called {
            panic!("CapnpAlloc::allocate_segment called multiple times");
        }

        let size = (size_words * 8) as usize;

        self.called = true;

        if size > self.buf.len() {
            panic!("Not enough space in CapnpAlloc");
        }

        (self.buf.as_mut_ptr(), (self.buf.len() / 8) as u32)
    }

    #[inline]
    unsafe fn deallocate_segment(&mut self, _ptr: *mut u8, _word_size: u32, _words_used: u32) {}
}

impl<'a> CapnpBorrowAlloc<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        // zero the buffer
        buf.fill(0);

        // safety: buffer is zeroed
        unsafe { Self::new_assert_zeroed(buf) }
    }

    pub unsafe fn new_assert_zeroed(buf: &'a mut [u8]) -> Self {
        #[cfg(debug_assertions)]
        {
            if !buf.iter().all(|&x| x == 0) {
                panic!("CapnpBorrowAlloc buffer must be zeroed");
            }
        }

        Self { buf, called: false }
    }
}

pub struct CapnpHeapAlloc {
    next_size: u32,
}

unsafe impl Allocator for CapnpHeapAlloc {
    #[inline]
    fn allocate_segment(&mut self, size_words: u32) -> (*mut u8, u32) {
        let size = size_words.max(self.next_size);
        let layout = Layout::from_size_align(size as usize * BYTES_PER_WORD, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        self.next_size += size;
        (ptr, size)
    }

    #[inline]
    unsafe fn deallocate_segment(&mut self, ptr: *mut u8, word_size: u32, _words_used: u32) {
        let layout = Layout::from_size_align(word_size as usize * BYTES_PER_WORD, 8).unwrap();

        unsafe {
            std::alloc::dealloc(ptr, layout);
        }
        self.next_size = 128; // reset to initial size
    }
}

impl CapnpHeapAlloc {
    pub fn new() -> Self {
        Self {
            next_size: 128, // 1 kb (128 words)
        }
    }
}

impl Default for CapnpHeapAlloc {
    fn default() -> Self {
        Self::new()
    }
}

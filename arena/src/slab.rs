pub(super) struct Slab<T> {
    items: Vec<T>,
}

impl<T> Slab<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() == self.items.capacity()
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn push(&mut self, value: T) -> *mut T {
        debug_assert!(!self.is_full());
        if self.is_full() {
            unsafe { core::hint::unreachable_unchecked() }
        }
        self.items.push(value);
        unsafe { self.items.last_mut().unwrap_unchecked() }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn start_ptr(&mut self) -> *mut T {
        unsafe { self.items.as_mut_ptr() }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn end_ptr(&mut self) -> *mut T {
        unsafe { self.items.as_mut_ptr().add(self.items.len()) }
    }

    pub fn extend<I: Iterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter)
    }

    pub fn split_off_extend(&mut self, len: usize, other: &mut Slab<T>) {
        debug_assert!(len <= self.len());
        debug_assert!(other.items.is_empty());

        let items = self.items.drain(len..);

        other.items.reserve(items.len() + 1);

        other.items.extend(items);
    }
}

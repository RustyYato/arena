use std::cell::{Cell, UnsafeCell};

use crate::slab::Slab;

pub struct Arena<T> {
    slabs: UnsafeCell<Vec<Slab<T>>>,
    full_slabs: Cell<usize>,
    len: Cell<usize>,
}

impl<T> Arena<T> {
    pub const fn new() -> Self {
        Self {
            slabs: UnsafeCell::new(Vec::new()),
            full_slabs: Cell::new(0),
            len: Cell::new(0),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len.get()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cold]
    #[inline(never)]
    #[allow(clippy::mut_from_ref)]
    fn insert_new_slab(&self, value: T) -> &mut T {
        let mut slab = self.prepare_slab();
        let value = unsafe { slab.push(value) };

        let slabs = unsafe { &mut *self.slabs.get() };
        slabs.push(slab);
        unsafe { &mut *value }
    }

    #[cold]
    #[inline(never)]
    #[allow(clippy::mut_from_ref)]
    fn prepare_slab(&self) -> Slab<T> {
        let slabs = unsafe { &mut *self.slabs.get() };
        Slab::new(1 << (5 + slabs.len()))
    }

    #[must_use]
    #[allow(clippy::mut_from_ref)]
    pub fn insert(&self, value: T) -> &mut T {
        self.len.set(self.len.get() + 1);
        let slabs = unsafe { &mut *self.slabs.get() };

        let maybe_not_full_slabs = unsafe { slabs.get_unchecked_mut(self.full_slabs.get()..) };

        for slab in maybe_not_full_slabs {
            if !slab.is_full() {
                let value = unsafe { slab.push(value) };
                return unsafe { &mut *value };
            }

            self.full_slabs.set(self.full_slabs.get() + 1);
        }

        self.insert_new_slab(value)
    }

    #[cold]
    #[allow(clippy::mut_from_ref)]
    fn insert_all_new_slab<I>(
        &self,
        previous_slab: Option<((usize, usize), T)>,
        iter: I,
    ) -> &mut [T]
    where
        I: Iterator<Item = T>,
    {
        let slabs = unsafe { &mut *self.slabs.get() };

        let slab_index = slabs.len();
        slabs.push(Slab::new(1 << (5 + slabs.len())));

        let slabs = slabs.as_mut_ptr();
        let slab = unsafe { &mut *slabs.add(slab_index) };

        let ptr = slab.end_ptr();

        if let Some(((prev, prev_len), value)) = previous_slab {
            let prev_slab = unsafe { &mut *slabs.add(prev) };

            prev_slab.split_off_extend(prev_len, slab);
            unsafe { slab.push(value) };
            slab.extend(iter);
        }

        let len = slab.len();

        self.len.set(self.len.get() + len);
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    ///
    /// # Safety
    ///
    /// the iterator may not call any extend or insert on this Arena
    #[must_use]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn insert_all<I: IntoIterator<Item = T>>(&self, iter: I) -> &mut [T] {
        let slabs = unsafe { &mut *self.slabs.get() };

        let full_slabs = self.full_slabs.get();
        let maybe_not_full_slabs = unsafe { slabs.get_unchecked_mut(full_slabs..) };

        let mut iter = iter.into_iter();

        let mut total_len = self.len.get();

        for (i, slab) in maybe_not_full_slabs.iter_mut().enumerate().rev() {
            let i = i + full_slabs;

            if slab.is_full() {
                continue;
            }

            let end = slab.end_ptr();
            let len = slab.len();

            let value = iter.try_for_each(|value| {
                if slab.is_full() {
                    return Err(value);
                }

                total_len += 1;
                unsafe { slab.push(value) };

                Ok(())
            });

            return if let Err(value) = value {
                return self.insert_all_new_slab(Some(((i, len), value)), iter);
            } else {
                let new_end = slab.end_ptr();
                let len = unsafe { new_end.offset_from(end) as usize };
                self.len.set(total_len);

                unsafe { core::slice::from_raw_parts_mut(end, len) }
            };
        }

        self.insert_all_new_slab(None, iter)
    }
}

#![forbid(unsafe_op_in_unsafe_fn)]

mod local;
mod slab;

pub use local::Arena;

pub struct ArenaRef<'a, T> {
    arena: &'a Arena<T>,
}

impl<'a, T> ArenaRef<'a, T> {
    pub fn new(arena: &'a mut Arena<T>) -> Self {
        Self { arena }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn as_ref(&mut self) -> ArenaRef<'_, T> {
        Self { arena: self.arena }
    }

    #[must_use]
    pub fn insert(&mut self, value: T) -> &'a mut T {
        self.arena.insert(value)
    }

    #[must_use]
    pub fn insert_all<I: IntoIterator<Item = T>>(&mut self, iter: I) -> &'a mut [T] {
        unsafe { self.arena.insert_all(iter) }
    }
}

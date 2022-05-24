use crate::local;

pub struct Arena<'id, T> {
    arena: local::Arena<T>,
    _id: generativity::Id<'id>,
}

pub struct Token<'id> {
    _id: generativity::Id<'id>,
}

impl<'id, T> Arena<'id, T> {
    pub fn new(guard: generativity::Guard<'id>) -> (Self, Token<'id>) {
        let id = guard.into();
        (
            Self {
                arena: local::Arena::new(),
                _id: id,
            },
            Token { _id: id },
        )
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    #[must_use]
    #[allow(clippy::mut_from_ref)]
    pub fn insert(&self, _: &Token<'id>, value: T) -> &mut T {
        self.arena.insert(value)
    }

    #[must_use]
    #[allow(clippy::mut_from_ref)]
    pub fn insert_all<I: IntoIterator<Item = T>>(&self, _: &mut Token<'id>, value: I) -> &mut [T] {
        unsafe { self.arena.insert_all(value) }
    }
}

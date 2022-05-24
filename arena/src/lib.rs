#![forbid(unsafe_op_in_unsafe_fn)]

pub mod local;
pub mod local_bulk;
mod slab;

pub use generativity::make_guard;

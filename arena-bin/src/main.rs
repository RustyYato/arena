fn main() {
    let mut arena = arena::Arena::new();
    let mut arena = arena::ArenaRef::new(&mut arena);

    let a = arena.insert_all(0..100);
    dbg!(a);
}

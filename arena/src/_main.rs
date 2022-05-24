#![feature(bench_black_box)]

use std::{hint::black_box, ops::Shl};

use arena::local_bulk::Arena;
use rand::{Rng, SeedableRng};

fn main() {
    arena::make_guard!(guard);
    let (arena, mut token) = Arena::<u32>::new(guard);

    let mut rng = rand::rngs::SmallRng::from_entropy();

    let start = std::time::Instant::now();
    for _ in 0..1 << 24 {
        let i: u32 = rng.gen();
        let iter = 0..i % 31;
        let _b = arena.insert_all(&mut token, iter.clone());
    }
    println!("{}", arena.len());
    println!("{}", 1 << 24);
    println!(
        "{:.2}",
        start.elapsed().as_secs_f64() / arena.len() as f64 * 1e9
    );
    println!(
        "{:.2}",
        start.elapsed().as_secs_f64() / (1 << 24) as f64 * 1e9
    );
}

extern crate cyd;
use cyd::search;

use criterion::{criterion_group, criterion_main, Criterion};
use pleco::{Board, Player};

fn bench_search_start_pos(c: &mut Criterion) {
    let board = Board::start_pos();
    c.bench_function("search depth 1 start position", |b| {
        b.iter(|| search(board.clone(), 1, Player::White))
    });

    c.bench_function("search depth 2 start position", |b| {
        b.iter(|| search(board.clone(), 2, Player::White))
    });
    
    c.bench_function("search depth 3 start position", |b| {
        b.iter(|| search(board.clone(), 3, Player::White))
    });
}

criterion_group!(benches, bench_search_start_pos);
criterion_main!(benches);

mod eval;
mod search;

use criterion::criterion_main;

criterion_main!(eval::eval_benches, search::search_benches);
criterion_main!(search::search_benches);

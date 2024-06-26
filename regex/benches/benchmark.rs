use criterion::{criterion_group, criterion_main, Criterion};
use regex::do_matching;
use std::time::Duration;

/// （計測のid、a?^n a^n という正規表現、文字列）というタプル。
const INPUTS: &[(&str, &str, &str)] = &[
    ("n = 2", "a?a?aa", "aa"),
    ("n = 4", "a?a?a?a?aaaa", "aaaa"),
    ("n = 6", "a?a?a?a?a?a?aaaaaa", "aaaaaa"),
    ("n = 8", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    ("n = 10", "a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa", "aaaaaaaaaa"),
];

fn depth_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Depth First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2,), |b, args| {
            b.iter(|| do_matching(args.0, args.1, true))
        });
    }
}
fn with_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("With First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2,), |b, args| {
            b.iter(|| do_matching(args.0, args.1, false))
        });
    }
}
criterion_group!(benches, with_first, depth_first);
criterion_main!(benches);
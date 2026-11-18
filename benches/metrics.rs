//! Criterion benchmarks for representative metric functions.
//!
//! Every ratio metric is O(1) arithmetic; the percentile helpers sort their
//! input, so they are benchmarked at several sizes (with throughput) to show
//! O(n log n) scaling. Money-based functions are benchmarked separately
//! because `rusty_money` arithmetic is decimal, not `f64`.
//!
//! Run with `cargo bench`; HTML reports land in `target/criterion/`.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use digital_health::{
    bed_day_cost_avoidance::bed_day_cost_avoided, digital_referral_turnaround_time as referral,
    econsult_turnaround_time as econsult, net_promoter_score::net_promoter_score,
    readmission_rate::readmission_rate, return_on_investment::roi_percent,
    secure_messaging_response_time as messaging, system_usability_scale::sus_score,
    telehealth_visit_rate::telehealth_visit_rate,
};
use rusty_money::{Money, iso};
use std::hint::black_box;

fn ratios(c: &mut Criterion) {
    let mut group = c.benchmark_group("ratios");
    group.bench_function("readmission_rate", |b| {
        b.iter(|| readmission_rate(black_box(400.0), black_box(2_500.0)));
    });
    group.bench_function("net_promoter_score", |b| {
        b.iter(|| net_promoter_score(black_box(60.0), black_box(15.0), black_box(100.0)));
    });
    group.bench_function("roi_percent", |b| {
        b.iter(|| roi_percent(black_box(150_000.0), black_box(100_000.0)));
    });
    group.bench_function("telehealth_visit_rate", |b| {
        b.iter(|| telehealth_visit_rate(black_box(320.0), black_box(1_680.0)));
    });
    group.bench_function("sus_score", |b| {
        b.iter(|| {
            sus_score(black_box([
                4.0, 2.0, 4.0, 2.0, 5.0, 1.0, 5.0, 1.0, 4.0, 2.0,
            ]))
        });
    });
    group.bench_function("zero_denominator_none", |b| {
        b.iter(|| readmission_rate(black_box(1.0), black_box(0.0)));
    });
    group.finish();
}

fn money(c: &mut Criterion) {
    let rate = Money::from_major(500, iso::USD);
    c.bench_function("money/bed_day_cost_avoided", |b| {
        b.iter(|| bed_day_cost_avoided(black_box(300), black_box(rate)));
    });
}

fn percentile_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("percentile");
    for n in [100_u32, 10_000, 1_000_000] {
        // Deterministic pseudo-shuffled input, so the sort has real work to do.
        let data: Vec<f64> = (0..n)
            .map(|i| f64::from(i.wrapping_mul(2_654_435_761) % n))
            .collect();
        group.throughput(Throughput::Elements(u64::from(n)));
        group.bench_with_input(BenchmarkId::new("referral", n), &data, |b, d| {
            b.iter(|| referral::percentile(black_box(d), black_box(90.0)));
        });
        group.bench_with_input(BenchmarkId::new("messaging", n), &data, |b, d| {
            b.iter(|| messaging::percentile(black_box(d), black_box(90.0)));
        });
        group.bench_with_input(BenchmarkId::new("econsult", n), &data, |b, d| {
            b.iter(|| econsult::percentile(black_box(d), black_box(90.0)));
        });
    }
    group.finish();
}

criterion_group!(benches, ratios, money, percentile_scaling);
criterion_main!(benches);

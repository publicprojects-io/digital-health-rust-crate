//! Cross-validation: each function is compared with an independent
//! re-implementation of its published formula over a grid of inputs.
//!
//! The reference implementations below are written from the published
//! definitions (Brooke 1996 for SUS, Reichheld 2003 for NPS, the linear
//! interpolation "type 7" quantile used by R and `NumPy`), not from the crate.

use digital_health::{
    digital_referral_turnaround_time::percentile, net_promoter_score::net_promoter_score,
    readmission_rate::readmission_rate, system_usability_scale::sus_score,
};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}

/// Brooke (1996): odd items contribute (x - 1), even items (5 - x); sum × 2.5.
fn sus_reference(r: &[f64; 10]) -> f64 {
    let sum: f64 = r
        .iter()
        .enumerate()
        .map(|(i, &x)| if i % 2 == 0 { x - 1.0 } else { 5.0 - x })
        .sum();
    sum * 2.5
}

#[test]
fn sus_matches_brooke_over_all_uniform_and_alternating_patterns() {
    for a in 1..=5 {
        for b in 1..=5 {
            let mut r = [0.0; 10];
            for (i, slot) in r.iter_mut().enumerate() {
                *slot = f64::from(if i % 2 == 0 { a } else { b });
            }
            assert!(close(sus_score(r), sus_reference(&r)), "{r:?}");
        }
    }
}

#[test]
fn nps_matches_reichheld_over_grid() {
    for n in 1..=40_u32 {
        for p in 0..=n {
            for d in 0..=(n - p) {
                let (pf, df, nf) = (f64::from(p), f64::from(d), f64::from(n));
                let reference = (pf / nf - df / nf) * 100.0;
                let got = net_promoter_score(pf, df, nf).unwrap();
                assert!(close(got, reference), "p={p} d={d} n={n}");
            }
        }
    }
}

/// Type-7 quantile: h = (n - 1)p/100; interpolate between floor and ceil.
// h is non-negative and below `len`, and the test data is tiny, so the casts are exact.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn type7(data: &[f64], p: f64) -> f64 {
    let mut v = data.to_vec();
    v.sort_by(f64::total_cmp);
    let h = (v.len() as f64 - 1.0) * p / 100.0;
    let lo = h.floor() as usize;
    let hi = h.ceil() as usize;
    v[lo] + (h - h.floor()) * (v[hi] - v[lo])
}

#[test]
fn percentile_matches_type7_over_grid() {
    for n in 1..=25_u32 {
        let data: Vec<f64> = (0..n).map(|i| f64::from((i * 7 + 3) % 31) * 1.5).collect();
        for p in (0..=100).step_by(5) {
            let p = f64::from(p);
            let got = percentile(&data, p).unwrap();
            assert!(close(got, type7(&data, p)), "n={n} p={p}");
        }
    }
}

#[test]
fn readmission_rate_matches_definition_over_grid() {
    for denom in 1..=60_u32 {
        for num in 0..=denom {
            let got = readmission_rate(f64::from(num), f64::from(denom)).unwrap();
            assert!(close(got, f64::from(num) / f64::from(denom) * 100.0));
        }
    }
}

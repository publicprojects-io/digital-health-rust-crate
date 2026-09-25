//! Private helpers shared across metric modules. Not part of the public API
//! — each public `percentile` function documents its own contract and
//! delegates here so the interpolation logic exists in exactly one place.

/// Empirical percentile (0–100) of a set of values, by linear interpolation
/// between order statistics. Sorts a copy of `values` with `f64::total_cmp`
/// (a total order over all `f64`, including NaN) so the sort itself can't
/// panic. Returns `None` for an empty slice.
///
/// # Casts
///
/// Callers pass at most a few years' worth of individual measurements, so
/// the slice length and the rank derived from it stay far below `2^52`; the
/// `f64 ↔ usize` conversions below cannot lose precision or sign in
/// practice.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub(crate) fn percentile(values: &[f64], p: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);

    // `rank` is a fractional index into `sorted`: e.g. p=50 on 10 elements
    // lands at rank 4.5, halfway between the 5th and 6th order statistics.
    let rank = (p / 100.0).clamp(0.0, 1.0) * (sorted.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    let frac = rank - lo as f64;
    // Linear interpolation between the two bracketing order statistics.
    Some(sorted[lo] + (sorted[hi] - sorted[lo]) * frac)
}

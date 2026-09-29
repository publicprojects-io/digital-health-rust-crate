//! # Readmission Rate
//!
//! The 30-day readmission rate is the share of index hospital discharges
//! followed by an unplanned readmission within 30 days. It is the standard
//! outcome measure for transitional-care and remote-monitoring programmes,
//! and directly tied to payer penalties and savings. The readmission
//! reduction is the relative fall in that rate against a baseline.
//!
//! ## How it's calculated
//!
//! ```text
//! Readmission rate = readmissions within 30 days / index discharges × 100
//!
//! Readmission reduction = (baseline rate − current rate) / baseline rate × 100
//! ```
//!
//! ## Why it matters
//!
//! Lower readmission is the clearest signal payers accept that a digital
//! programme improved care rather than merely engaged patients. Reporting
//! the reduction as a *relative* change against a stated baseline makes a
//! programme comparable across hospitals whose starting rates differ.
//!
//! ## Worked example
//!
//! A heart-failure remote-monitoring programme has 2,500 index discharges
//! in a year, 400 of which are followed by an unplanned readmission within
//! 30 days. The prior year's rate for the same population was 16%; this
//! year's improves to 13.6%.
//!
//! ```rust
//! use digital_health::readmission_rate::{readmission_rate, readmission_reduction};
//!
//! // 400 / 2,500 × 100 = 16%.
//! let rate = readmission_rate(400.0, 2_500.0).unwrap();
//! assert!((rate - 16.0).abs() < 1e-9);
//!
//! // (16 − 13.6) / 16 × 100 = 15% relative reduction.
//! let reduction = readmission_reduction(16.0, 13.6).unwrap();
//! assert!((reduction - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Use the hospital's discharge and admission records, applying the same
//! exclusions each period — planned readmissions, transfers, and in-hospital
//! deaths are typically excluded from index discharges or readmissions per
//! the measure specification being followed.
//!
//! ## Pitfalls
//!
//! - **Comparing unadjusted rates across hospitals** — case mix drives
//!   readmission; official measures are risk-adjusted, this one is not.
//! - **Changing the measure definition between periods** — a different
//!   exclusion list makes the baseline and current rate incomparable.
//! - **Reading a drop as programme effect** — regression to the mean and
//!   secular trends also lower rates; a comparison group is stronger.
//!
//! ## Sources
//!
//! - CMS Hospital Readmissions Reduction Program (HRRP) measure
//!   methodology.
//! - Peer-reviewed literature on remote patient monitoring and
//!   transitional-care effects on 30-day readmission.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The 30-day readmission rate: unplanned readmissions within 30 days as a
/// percentage of index discharges.
///
/// # Arguments
///
/// * `readmissions` — index discharges followed by an unplanned readmission
///   within 30 days (count).
/// * `index_discharges` — total eligible index discharges in the period
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `index_discharges` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::readmission_rate::readmission_rate;
///
/// // Worked example: 400 / 2,500 × 100 = 16%.
/// let rate = readmission_rate(400.0, 2_500.0).unwrap();
/// assert!((rate - 16.0).abs() < 1e-9);
///
/// assert!(readmission_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn readmission_rate(readmissions: f64, index_discharges: f64) -> Option<f64> {
    if index_discharges == 0.0 {
        None
    } else {
        Some(readmissions / index_discharges * 100.0)
    }
}

/// The readmission reduction: the relative fall from a baseline readmission
/// rate to a current one, as a percentage of the baseline.
///
/// # Arguments
///
/// * `baseline_rate` — the baseline readmission rate (percentage).
/// * `current_rate` — the current readmission rate (percentage).
///
/// # Returns
///
/// `Some(percentage)` — positive for a reduction, negative for an
/// increase — or `None` if `baseline_rate` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::readmission_rate::readmission_reduction;
///
/// // Worked example: (16 − 13.6) / 16 × 100 = 15%.
/// let reduction = readmission_reduction(16.0, 13.6).unwrap();
/// assert!((reduction - 15.0).abs() < 1e-9);
///
/// assert!(readmission_reduction(0.0, 5.0).is_none());
/// ```
#[must_use]
pub fn readmission_reduction(baseline_rate: f64, current_rate: f64) -> Option<f64> {
    if baseline_rate == 0.0 {
        None
    } else {
        Some((baseline_rate - current_rate) / baseline_rate * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "400 ... readmission within 30 days" of 2,500 — 16%.
    #[test]
    fn worked_example_readmission_rate_is_16_percent() {
        let rate = readmission_rate(400.0, 2_500.0).unwrap();
        assert!((rate - 16.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "16% ... improves to 13.6%" — 15% relative reduction.
    #[test]
    fn worked_example_readmission_reduction_is_15_percent() {
        let reduction = readmission_reduction(16.0, 13.6).unwrap();
        assert!((reduction - 15.0).abs() < 1e-9, "got {reduction}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(readmission_rate(1.0, 0.0).is_none());
        assert!(readmission_reduction(0.0, 5.0).is_none());
    }
}

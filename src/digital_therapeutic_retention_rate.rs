//! # Digital Therapeutic Retention Rate
//!
//! Digital therapeutic retention rate is the share of patients enrolled in
//! a digital therapeutic (including prescription digital therapeutics, or
//! PDTs) who are still actively using it at day 7 and at day 30. A digital
//! therapeutic only produces its clinical effect if patients keep using it
//! long enough to complete the programme, so early-attrition checkpoints
//! are tracked as a measure distinct from clinical outcomes among patients
//! who do stick with it.
//!
//! ## How it's calculated
//!
//! ```text
//! Day-7 retention rate = active users at day 7 / enrolled users × 100
//!
//! Day-30 retention rate = active users at day 30 / enrolled users × 100
//! ```
//!
//! ## Why it matters
//!
//! Day-7 and day-30 retention are the standard early-attrition checkpoints
//! in digital therapeutics literature. A steep drop between the two — a
//! product with strong initial uptake but weak sustained engagement — is a
//! different problem than low day-7 retention in the first place, and needs
//! a different fix (engagement design versus onboarding design). Reporting
//! only one checkpoint, or only an average "active users" figure, hides
//! which of the two is actually happening.
//!
//! ## Worked example
//!
//! A digital therapeutic for insomnia enrols 1,000 patients. 640 are still
//! actively using it at day 7; 280 are still active at day 30.
//!
//! ```rust
//! use digital_health::digital_therapeutic_retention_rate::{day_7_retention_rate, day_30_retention_rate};
//!
//! // 640 / 1,000 × 100 = 64%.
//! let day_7 = day_7_retention_rate(640.0, 1_000.0).unwrap();
//! assert!((day_7 - 64.0).abs() < 1e-9);
//!
//! // 280 / 1,000 × 100 = 28%.
//! let day_30 = day_30_retention_rate(280.0, 1_000.0).unwrap();
//! assert!((day_30 - 28.0).abs() < 1e-9);
//!
//! // The steep drop from 64% to 28% is the signal that the product's
//! // initial hook works but its sustained engagement design doesn't.
//! assert!(day_30 < day_7);
//! ```
//!
//! ## Data sources and caveats
//!
//! The digital therapeutic's own usage analytics are the primary source;
//! "active" must be defined consistently — a therapeutic session actually
//! completed is a stronger signal than merely opening the app — and stated
//! alongside the figure.
//!
//! ## Pitfalls
//!
//! - **Reporting a single retention number without the day-7-versus-day-30
//!   comparison** — the comparison is what reveals the attrition pattern;
//!   either checkpoint alone hides it.
//! - **Defining "active" inconsistently** — an app-open event and a
//!   completed therapeutic session are very different engagement signals
//!   and should not be conflated.
//! - **Not segmenting retention by patient subgroup** — digital-engagement
//!   dropoff often correlates with the same digital-exclusion factors
//!   flagged elsewhere in this crate, for example in [patient portal
//!   adoption rate](crate::patient_portal_adoption_rate).
//!
//! ## Sources
//!
//! - Peer-reviewed literature on digital therapeutics engagement and
//!   retention, including studies published in JMIR mHealth and uHealth.
//! - FDA guidance on prescription digital therapeutics.
//! - Industry benchmarking publications on mobile health app retention
//!   curves.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The day-7 retention rate: patients still actively using the digital
/// therapeutic at day 7, as a percentage of enrolled patients.
///
/// # Arguments
///
/// * `active_at_day_7` — enrolled patients still actively using the
///   therapeutic at day 7 (count).
/// * `enrolled` — total patients enrolled in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `enrolled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_therapeutic_retention_rate::day_7_retention_rate;
///
/// // Worked example: 640 / 1,000 × 100 = 64%.
/// let rate = day_7_retention_rate(640.0, 1_000.0).unwrap();
/// assert!((rate - 64.0).abs() < 1e-9);
///
/// assert!(day_7_retention_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn day_7_retention_rate(active_at_day_7: f64, enrolled: f64) -> Option<f64> {
    if enrolled == 0.0 {
        None
    } else {
        Some(active_at_day_7 / enrolled * 100.0)
    }
}

/// The day-30 retention rate: patients still actively using the digital
/// therapeutic at day 30, as a percentage of enrolled patients.
///
/// # Arguments
///
/// * `active_at_day_30` — enrolled patients still actively using the
///   therapeutic at day 30 (count).
/// * `enrolled` — total patients enrolled in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `enrolled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_therapeutic_retention_rate::day_30_retention_rate;
///
/// // Worked example: 280 / 1,000 × 100 = 28%.
/// let rate = day_30_retention_rate(280.0, 1_000.0).unwrap();
/// assert!((rate - 28.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn day_30_retention_rate(active_at_day_30: f64, enrolled: f64) -> Option<f64> {
    if enrolled == 0.0 {
        None
    } else {
        Some(active_at_day_30 / enrolled * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "640 are still actively using it at day 7" — 64%.
    #[test]
    fn worked_example_day_7_retention_is_64_percent() {
        let rate = day_7_retention_rate(640.0, 1_000.0).unwrap();
        assert!((rate - 64.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "280 are still active at day 30" — 28%.
    #[test]
    fn worked_example_day_30_retention_is_28_percent() {
        let rate = day_30_retention_rate(280.0, 1_000.0).unwrap();
        assert!((rate - 28.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(day_7_retention_rate(1.0, 0.0).is_none());
        assert!(day_30_retention_rate(1.0, 0.0).is_none());
    }
}

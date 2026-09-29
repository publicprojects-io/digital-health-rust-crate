//! # Long-Term Retention Rate
//!
//! Long-term retention rate is the share of an enrolment cohort still
//! interacting with a digital intervention at day 60 and day 90 — the
//! extension of the day-7 and day-30 checkpoints in [digital therapeutic
//! retention rate](crate::digital_therapeutic_retention_rate) to the
//! horizons where clinical benefit is decided.
//!
//! ## How it's calculated
//!
//! ```text
//! Day-60 retention = cohort users active at day 60 / cohort × 100
//!
//! Day-90 retention = cohort users active at day 90 / cohort × 100
//! ```
//!
//! ## Why it matters
//!
//! Early retention shows onboarding worked; retention at 60 and 90 days
//! shows the product earned a place in the patient's routine, which is
//! where most outcome trials find the benefit. A steep fall between day 30
//! and day 60 is a distinct failure from poor onboarding.
//!
//! ## Worked example
//!
//! A cohort of 1,000 patients enrols in a month. 540 are still active at
//! day 60 and 410 at day 90.
//!
//! ```rust
//! use digital_health::long_term_retention_rate::{day_60_retention_rate, day_90_retention_rate};
//!
//! // 540 / 1,000 × 100 = 54% at day 60.
//! let day_60_retention_rate_value = day_60_retention_rate(540.0, 1_000.0).unwrap();
//! assert!((day_60_retention_rate_value - 54.0).abs() < 1e-9);
//!
//! // 410 / 1,000 × 100 = 41% at day 90.
//! let day_90_retention_rate_value = day_90_retention_rate(410.0, 1_000.0).unwrap();
//! assert!((day_90_retention_rate_value - 41.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Use a fixed cohort by enrolment date and define "retained" as a
//! meaningful action in a window around the day, not merely not having
//! disenrolled. Report the cohort size beside every rate.
//!
//! ## Pitfalls
//!
//! - Mixing cohorts of different enrolment dates within a single rate.
//! - Treating retention as a survival curve — a patient who returns after a
//!   gap is retained at day 90 but not at day 60.
//! - Reporting only rates for cohorts old enough to reach the horizon.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on retention and attrition in digital health
//!   interventions.
//! - Eysenbach, G. "The Law of Attrition." *Journal of Medical Internet
//!   Research*, 2005.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The day-60 retention rate: cohort users active at day 60 as a percentage
/// of the cohort.
///
/// # Arguments
///
/// * `retained_at_day_60` — cohort users active at day 60 (count).
/// * `cohort` — users in the enrolment cohort (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `cohort` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::long_term_retention_rate::day_60_retention_rate;
///
/// // Worked example: 540 / 1,000 × 100 = 54%.
/// let value = day_60_retention_rate(540.0, 1_000.0).unwrap();
/// assert!((value - 54.0).abs() < 1e-9);
///
/// assert!(day_60_retention_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn day_60_retention_rate(retained_at_day_60: f64, cohort: f64) -> Option<f64> {
    if cohort == 0.0 {
        None
    } else {
        Some(retained_at_day_60 / cohort * 100.0)
    }
}

/// The day-90 retention rate: cohort users active at day 90 as a percentage
/// of the cohort.
///
/// # Arguments
///
/// * `retained_at_day_90` — cohort users active at day 90 (count).
/// * `cohort` — users in the enrolment cohort (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `cohort` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::long_term_retention_rate::day_90_retention_rate;
///
/// // Worked example: 410 / 1,000 × 100 = 41%.
/// let value = day_90_retention_rate(410.0, 1_000.0).unwrap();
/// assert!((value - 41.0).abs() < 1e-9);
///
/// assert!(day_90_retention_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn day_90_retention_rate(retained_at_day_90: f64, cohort: f64) -> Option<f64> {
    if cohort == 0.0 {
        None
    } else {
        Some(retained_at_day_90 / cohort * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "540 / 1,000 × 100 = 54% at day 60."
    #[test]
    fn worked_example_day_60_retention_rate() {
        let value = day_60_retention_rate(540.0, 1_000.0).unwrap();
        assert!((value - 54.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "410 / 1,000 × 100 = 41% at day 90."
    #[test]
    fn worked_example_day_90_retention_rate() {
        let value = day_90_retention_rate(410.0, 1_000.0).unwrap();
        assert!((value - 41.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(day_60_retention_rate(1.0, 0.0).is_none());
        assert!(day_90_retention_rate(1.0, 0.0).is_none());
    }
}

//! # Clinician Documentation Time
//!
//! Clinician documentation time is the average time a clinician spends in
//! the electronic health record documenting per encounter, and its
//! reduction after a change such as ambient scribing, templates, or
//! in-basket redesign. It is the most direct operational proxy for the
//! paperwork burden behind physician burnout.
//!
//! ## How it's calculated
//!
//! ```text
//! Documentation minutes per encounter = total documentation minutes / encounters
//!
//! Documentation time reduction = (baseline − current) / baseline × 100
//! ```
//!
//! ## Why it matters
//!
//! Time-and-motion and EHR-audit-log studies consistently find that
//! clinicians spend a large share of the working day on EHR documentation,
//! and that this burden is tied to burnout. A per-encounter figure
//! normalises for volume, so a busy clinic isn't mistaken for a
//! documentation-heavy one, and a relative reduction shows whether a tool
//! actually gave time back.
//!
//! ## Worked example
//!
//! A clinic's EHR audit logs record 240,000 minutes of clinician
//! documentation time across 12,000 encounters in a quarter — after an
//! ambient-scribe pilot the same measure falls to 15 minutes per encounter.
//!
//! ```rust
//! use digital_health::clinician_documentation_time::{
//!     documentation_minutes_per_encounter, documentation_time_reduction,
//! };
//!
//! // 240,000 / 12,000 = 20 minutes per encounter.
//! let baseline = documentation_minutes_per_encounter(240_000.0, 12_000.0).unwrap();
//! assert!((baseline - 20.0).abs() < 1e-9);
//!
//! // (20 − 15) / 20 × 100 = 25% reduction.
//! let reduction = documentation_time_reduction(baseline, 15.0).unwrap();
//! assert!((reduction - 25.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! EHR vendor audit-log analytics (for example, active-use time measures)
//! are the most reliable source; self-report is cheaper but overstated.
//! Define once whether after-hours ("pajama time") documentation is
//! included, and keep it the same in the baseline and current periods.
//!
//! ## Pitfalls
//!
//! - **Counting time saved, not time returned** — a shorter note doesn't
//!   reduce burnout if the freed time is filled with more patients.
//! - **Mixing specialties** — documentation load per encounter differs
//!   widely between, say, primary care and dermatology.
//! - **Ignoring note quality** — faster documentation that degrades data
//!   quality has moved the burden, not removed it.
//!
//! ## Sources
//!
//! - Sinsky, C. et al. "Allocation of Physician Time in Ambulatory
//!   Practice." *Annals of Internal Medicine*, 2016.
//! - Arndt, B. G. et al. "Tethered to the EHR." *Annals of Family
//!   Medicine*, 2017.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The average documentation time per encounter, in minutes.
///
/// # Arguments
///
/// * `documentation_minutes` — total clinician EHR documentation time in
///   the period (minutes).
/// * `encounters` — total encounters in the period (count).
///
/// # Returns
///
/// `Some(minutes)`, or `None` if `encounters` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::clinician_documentation_time::documentation_minutes_per_encounter;
///
/// // Worked example: 240,000 / 12,000 = 20 minutes.
/// let minutes = documentation_minutes_per_encounter(240_000.0, 12_000.0).unwrap();
/// assert!((minutes - 20.0).abs() < 1e-9);
///
/// assert!(documentation_minutes_per_encounter(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn documentation_minutes_per_encounter(
    documentation_minutes: f64,
    encounters: f64,
) -> Option<f64> {
    if encounters == 0.0 {
        None
    } else {
        Some(documentation_minutes / encounters)
    }
}

/// The documentation time reduction: the relative fall from a baseline
/// per-encounter documentation time to a current one, as a percentage of
/// the baseline.
///
/// # Arguments
///
/// * `baseline_minutes` — baseline documentation minutes per encounter.
/// * `current_minutes` — current documentation minutes per encounter.
///
/// # Returns
///
/// `Some(percentage)` — positive for a reduction, negative for an
/// increase — or `None` if `baseline_minutes` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::clinician_documentation_time::documentation_time_reduction;
///
/// // Worked example: (20 − 15) / 20 × 100 = 25%.
/// let reduction = documentation_time_reduction(20.0, 15.0).unwrap();
/// assert!((reduction - 25.0).abs() < 1e-9);
///
/// assert!(documentation_time_reduction(0.0, 15.0).is_none());
/// ```
#[must_use]
pub fn documentation_time_reduction(baseline_minutes: f64, current_minutes: f64) -> Option<f64> {
    if baseline_minutes == 0.0 {
        None
    } else {
        Some((baseline_minutes - current_minutes) / baseline_minutes * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "240,000 minutes ... across 12,000 encounters" — 20 minutes.
    #[test]
    fn worked_example_minutes_per_encounter_is_20() {
        let minutes = documentation_minutes_per_encounter(240_000.0, 12_000.0).unwrap();
        assert!((minutes - 20.0).abs() < 1e-9, "got {minutes}");
    }

    // Doc line: "falls to 15 minutes per encounter" — 25% reduction.
    #[test]
    fn worked_example_documentation_time_reduction_is_25_percent() {
        let reduction = documentation_time_reduction(20.0, 15.0).unwrap();
        assert!((reduction - 25.0).abs() < 1e-9, "got {reduction}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(documentation_minutes_per_encounter(1.0, 0.0).is_none());
        assert!(documentation_time_reduction(0.0, 15.0).is_none());
    }
}

//! # Virtual Ward Bed Days
//!
//! Virtual ward bed days measures the hospital bed days saved by moving
//! part of a patient's stay — post-surgical recovery, for example — into a
//! virtual ward with remote monitoring, and the relative reduction in bed
//! days against a baseline.
//!
//! ## How it's calculated
//!
//! ```text
//! Bed days saved = patients × (conventional stay days − actual hospital stay days)
//!
//! Bed-day reduction = (baseline bed days − current bed days) / baseline bed days × 100
//! ```
//!
//! ## Why it matters
//!
//! Bed capacity is the scarcest resource in most hospitals, so bed days
//! freed is the operational headline for a virtual ward, and the quantity a
//! cost case is built on. For the currency amount see
//! [`bed_day_cost_avoidance`](crate::bed_day_cost_avoidance).
//!
//! ## Worked example
//!
//! 200 post-surgical patients would conventionally stay 6.0 days in
//! hospital; enrolled in a virtual ward they stay 4.5 days in hospital and
//! recover the rest at home. The baseline bed days of 1,200 fall to 900.
//!
//! ```rust
//! use digital_health::virtual_ward_bed_days::{bed_days_saved, bed_day_reduction_rate};
//!
//! // 200 × (6.0 − 4.5) = 300 bed days.
//! let bed_days_saved_value = bed_days_saved(200.0, 6.0, 4.5);
//! assert!((bed_days_saved_value - 300.0).abs() < 1e-9);
//!
//! // (1,200 − 900) / 1,200 × 100 = 25% reduction.
//! let bed_day_reduction_rate_value = bed_day_reduction_rate(1_200.0, 900.0).unwrap();
//! assert!((bed_day_reduction_rate_value - 25.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Hospital days come from admission and discharge records; virtual ward
//! days are not bed days and must not be counted as hospital days. The
//! conventional stay should be a matched historical or comparison-group
//! figure, not an average of all admissions.
//!
//! ## Pitfalls
//!
//! - Using an unmatched conventional stay length — it decides the entire
//!   result.
//! - Ignoring readmissions — bed days saved are illusory if the patient
//!   returns; see [`readmission_rate`](crate::readmission_rate).
//! - Counting virtual ward days as beds in a cost case without their own
//!   staffing cost.
//!
//! ## Sources
//!
//! - NHS England guidance on virtual wards (hospital at home) and bed-day
//!   measurement.
//! - Peer-reviewed literature on hospital-at-home and early-supported-
//!   discharge length-of-stay outcomes.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The bed days saved: patients multiplied by the conventional stay minus
/// the actual hospital stay.
///
/// # Arguments
///
/// * `patients` — patients enrolled in the virtual ward (count).
/// * `conventional_stay_days` — average hospital stay days without the
///   virtual ward.
/// * `actual_stay_days` — average hospital stay days with the virtual ward.
///
/// # Returns
///
/// The bed days saved — negative if the virtual ward lengthened hospital
/// stays. Always defined.
///
/// # Examples
///
/// ```rust
/// use digital_health::virtual_ward_bed_days::bed_days_saved;
///
/// // Worked example: 200 × (6.0 − 4.5) = 300.
/// let value = bed_days_saved(200.0, 6.0, 4.5);
/// assert!((value - 300.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn bed_days_saved(patients: f64, conventional_stay_days: f64, actual_stay_days: f64) -> f64 {
    patients * (conventional_stay_days - actual_stay_days)
}

/// The bed-day reduction rate: the relative fall from baseline bed days to
/// current bed days, as a percentage of the baseline.
///
/// # Arguments
///
/// * `baseline_bed_days` — bed days used before the virtual ward.
/// * `current_bed_days` — bed days used with the virtual ward.
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `baseline_bed_days` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::virtual_ward_bed_days::bed_day_reduction_rate;
///
/// // Worked example: (1,200 − 900) / 1,200 × 100 = 25%.
/// let value = bed_day_reduction_rate(1_200.0, 900.0).unwrap();
/// assert!((value - 25.0).abs() < 1e-9);
///
/// assert!(bed_day_reduction_rate(0.0, 1.0).is_none());
/// ```
#[must_use]
pub fn bed_day_reduction_rate(baseline_bed_days: f64, current_bed_days: f64) -> Option<f64> {
    if baseline_bed_days == 0.0 {
        None
    } else {
        Some((baseline_bed_days - current_bed_days) / baseline_bed_days * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "200 × (6.0 − 4.5) = 300 bed days."
    #[test]
    fn worked_example_bed_days_saved() {
        let value = bed_days_saved(200.0, 6.0, 4.5);
        assert!((value - 300.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "(1,200 − 900) / 1,200 × 100 = 25% reduction."
    #[test]
    fn worked_example_bed_day_reduction_rate() {
        let value = bed_day_reduction_rate(1_200.0, 900.0).unwrap();
        assert!((value - 25.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(bed_day_reduction_rate(0.0, 1.0).is_none());
    }
}

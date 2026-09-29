//! # Biometric Improvement
//!
//! Biometric improvement measures whether a digital programme moves a
//! clinical marker in the right direction: the relative fall in a value
//! such as `HbA1c` or BMI against baseline, the share of patients reaching
//! a clinical target, and the estimated A1c (eA1c) implied by a mean
//! glucose reading from a CGM or meter.
//!
//! ## How it's calculated
//!
//! ```text
//! Estimated A1c (%) = (mean glucose in mg/dL + 46.7) / 28.7
//!
//! Biometric reduction = (baseline − current) / baseline × 100
//!
//! Target attainment rate = patients at target / patients measured × 100
//! ```
//!
//! ## Why it matters
//!
//! Clinical outcomes are the baseline proof that a digital health platform
//! works. Reporting the relative reduction, and the share reaching target,
//! together shows both the size of the average effect and how many patients
//! it actually reached — an average can improve while few patients hit
//! their target.
//!
//! ## Worked example
//!
//! A diabetes programme's CGM users have a mean glucose of 154.2 mg/dL. The
//! cohort's mean `HbA1c` falls from 9.0% at baseline to 7.65% at six
//! months, and 420 of the 600 patients measured are under the 7% target.
//!
//! ```rust
//! use digital_health::biometric_improvement::{estimated_a1c, biometric_reduction, target_attainment_rate};
//!
//! // (154.2 + 46.7) / 28.7 = 7.0% eA1c.
//! let estimated_a1c_value = estimated_a1c(154.2);
//! assert!((estimated_a1c_value - 7.0).abs() < 1e-9);
//!
//! // (9.0 − 7.65) / 9.0 × 100 = 15% reduction.
//! let biometric_reduction_value = biometric_reduction(9.0, 7.65).unwrap();
//! assert!((biometric_reduction_value - 15.0).abs() < 1e-9);
//!
//! // 420 / 600 × 100 = 70% at target.
//! let target_attainment_rate_value = target_attainment_rate(420.0, 600.0).unwrap();
//! assert!((target_attainment_rate_value - 70.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Baseline and follow-up values should come from the same assay or device
//! type; the eA1c conversion is the ADAG regression and is an estimate, not
//! a laboratory `HbA1c`. Define the target threshold once, before
//! measuring.
//!
//! ## Pitfalls
//!
//! - Reporting an average change without the share reaching target — a few
//!   large improvers can mask a stalled majority.
//! - Comparing eA1c with laboratory `HbA1c` as if interchangeable —
//!   individual glycation differences make them diverge.
//! - Excluding patients lost to follow-up — completers-only reporting
//!   overstates improvement.
//!
//! ## Sources
//!
//! - Nathan, D. M. et al. (ADAG Study Group). "Translating the A1C Assay
//!   Into Estimated Average Glucose Values." *Diabetes Care*, 2008.
//! - ADA Standards of Care in Diabetes, on glycaemic targets.
//! - Peer-reviewed literature on digital programmes and `HbA1c` or BMI
//!   reduction.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The estimated A1c (eA1c) implied by a mean glucose reading, using the
/// ADAG regression.
///
/// # Arguments
///
/// * `mean_glucose_mg_dl` — mean glucose over the period (mg/dL).
///
/// # Returns
///
/// The estimated A1c as a percentage. Always defined — there is no
/// denominator.
///
/// # Examples
///
/// ```rust
/// use digital_health::biometric_improvement::estimated_a1c;
///
/// // Worked example: (154.2 + 46.7) / 28.7 = 7.0%.
/// let value = estimated_a1c(154.2);
/// assert!((value - 7.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn estimated_a1c(mean_glucose_mg_dl: f64) -> f64 {
    (mean_glucose_mg_dl + 46.7) / 28.7
}

/// The biometric reduction: the relative fall from a baseline value to a
/// current one, as a percentage of the baseline.
///
/// # Arguments
///
/// * `baseline` — baseline value of the marker, such as `HbA1c` or BMI.
/// * `current` — current value of the same marker.
///
/// # Returns
///
/// `Some(percentage)` — positive for an improvement (fall), negative for a
/// rise — or `None` if `baseline` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::biometric_improvement::biometric_reduction;
///
/// // Worked example: (9.0 − 7.65) / 9.0 × 100 = 15%.
/// let value = biometric_reduction(9.0, 7.65).unwrap();
/// assert!((value - 15.0).abs() < 1e-9);
///
/// assert!(biometric_reduction(0.0, 1.0).is_none());
/// ```
#[must_use]
pub fn biometric_reduction(baseline: f64, current: f64) -> Option<f64> {
    if baseline == 0.0 {
        None
    } else {
        Some((baseline - current) / baseline * 100.0)
    }
}

/// The target attainment rate: patients at their clinical target as a
/// percentage of patients measured.
///
/// # Arguments
///
/// * `at_target` — measured patients at or better than target (count).
/// * `measured` — patients with a measurement in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `measured` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::biometric_improvement::target_attainment_rate;
///
/// // Worked example: 420 / 600 × 100 = 70%.
/// let value = target_attainment_rate(420.0, 600.0).unwrap();
/// assert!((value - 70.0).abs() < 1e-9);
///
/// assert!(target_attainment_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn target_attainment_rate(at_target: f64, measured: f64) -> Option<f64> {
    if measured == 0.0 {
        None
    } else {
        Some(at_target / measured * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "(154.2 + 46.7) / 28.7 = 7.0% eA1c."
    #[test]
    fn worked_example_estimated_a1c() {
        let value = estimated_a1c(154.2);
        assert!((value - 7.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "(9.0 − 7.65) / 9.0 × 100 = 15% reduction."
    #[test]
    fn worked_example_biometric_reduction() {
        let value = biometric_reduction(9.0, 7.65).unwrap();
        assert!((value - 15.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "420 / 600 × 100 = 70% at target."
    #[test]
    fn worked_example_target_attainment_rate() {
        let value = target_attainment_rate(420.0, 600.0).unwrap();
        assert!((value - 70.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(biometric_reduction(0.0, 1.0).is_none());
        assert!(target_attainment_rate(1.0, 0.0).is_none());
    }
}

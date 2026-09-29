//! # Medication Adherence Rate
//!
//! Medication adherence rate measures whether patients take prescribed
//! medication as directed. The proportion of days covered (PDC) is the
//! share of days in a period a patient had medication on hand; the adherent
//! patient rate is the share of patients whose PDC reaches the conventional
//! 80% threshold.
//!
//! ## How it's calculated
//!
//! ```text
//! Proportion of days covered = days covered / days in period × 100
//!
//! Adherent patient rate = patients with PDC at or above 80% / patients × 100
//! ```
//!
//! ## Why it matters
//!
//! Non-adherence is among the largest preventable drivers of poor outcomes
//! and avoidable admissions, and digital reminders, connected pill packs,
//! and apps are its most common targets. PDC is the pharmacy-claims
//! standard used in quality programmes, and the 80% threshold turns it into
//! a cohort-level rate that can be tracked and compared.
//!
//! ## Worked example
//!
//! A patient has medication on hand for 72 of the 90 days after a first
//! fill. Across a cohort of 800 patients on a statin, 560 reach a PDC of
//! 80% or higher.
//!
//! ```rust
//! use digital_health::medication_adherence_rate::{proportion_of_days_covered, adherent_patient_rate};
//!
//! // 72 / 90 × 100 = 80% PDC.
//! let proportion_of_days_covered_value = proportion_of_days_covered(72.0, 90.0).unwrap();
//! assert!((proportion_of_days_covered_value - 80.0).abs() < 1e-9);
//!
//! // 560 / 800 × 100 = 70% adherent.
//! let adherent_patient_rate_value = adherent_patient_rate(560.0, 800.0).unwrap();
//! assert!((adherent_patient_rate_value - 70.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! PDC comes from pharmacy fill or claims data, counting overlapping fills
//! once; it measures supply, not ingestion. Connected-device data measures
//! ingestion more directly but covers fewer patients. Keep the measurement
//! window and threshold fixed across periods.
//!
//! ## Pitfalls
//!
//! - Treating a filled prescription as a taken dose — PDC is a supply
//!   proxy.
//! - Counting overlapping fills twice, which inflates days covered above
//!   the period length.
//! - Comparing across drug classes with different standard thresholds or
//!   refill patterns.
//!
//! ## Sources
//!
//! - Pharmacy Quality Alliance (PQA) adherence measure specifications, on
//!   PDC and the 80% threshold.
//! - CMS Medicare Part D Star Ratings medication adherence measures.
//! - Peer-reviewed literature on digital interventions for medication
//!   adherence.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The proportion of days covered (PDC): days a patient had medication on
/// hand as a percentage of days in the measurement period.
///
/// # Arguments
///
/// * `days_covered` — days in the period with medication on hand (count).
/// * `days_in_period` — total days in the measurement period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `days_in_period` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::medication_adherence_rate::proportion_of_days_covered;
///
/// // Worked example: 72 / 90 × 100 = 80%.
/// let value = proportion_of_days_covered(72.0, 90.0).unwrap();
/// assert!((value - 80.0).abs() < 1e-9);
///
/// assert!(proportion_of_days_covered(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn proportion_of_days_covered(days_covered: f64, days_in_period: f64) -> Option<f64> {
    if days_in_period == 0.0 {
        None
    } else {
        Some(days_covered / days_in_period * 100.0)
    }
}

/// The adherent patient rate: patients whose PDC meets the adherence
/// threshold, as a percentage of patients.
///
/// # Arguments
///
/// * `adherent_patients` — patients with PDC at or above the threshold
///   (count).
/// * `patients` — patients in the cohort (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `patients` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::medication_adherence_rate::adherent_patient_rate;
///
/// // Worked example: 560 / 800 × 100 = 70%.
/// let value = adherent_patient_rate(560.0, 800.0).unwrap();
/// assert!((value - 70.0).abs() < 1e-9);
///
/// assert!(adherent_patient_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn adherent_patient_rate(adherent_patients: f64, patients: f64) -> Option<f64> {
    if patients == 0.0 {
        None
    } else {
        Some(adherent_patients / patients * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "72 / 90 × 100 = 80% PDC."
    #[test]
    fn worked_example_proportion_of_days_covered() {
        let value = proportion_of_days_covered(72.0, 90.0).unwrap();
        assert!((value - 80.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "560 / 800 × 100 = 70% adherent."
    #[test]
    fn worked_example_adherent_patient_rate() {
        let value = adherent_patient_rate(560.0, 800.0).unwrap();
        assert!((value - 70.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(proportion_of_days_covered(1.0, 0.0).is_none());
        assert!(adherent_patient_rate(1.0, 0.0).is_none());
    }
}

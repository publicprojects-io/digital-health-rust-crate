//! # Medication Reconciliation Rate
//!
//! Medication reconciliation rate is the share of hospital admissions and
//! discharges for which a structured medication reconciliation is completed
//! against the electronic medication record. It is one of the
//! highest-yield digital medication-safety checks in this crate: omissions
//! and duplications at transitions of care are a leading cause of
//! preventable adverse drug events, and a digital reconciliation tool only
//! provides that protection if reconciliation is actually completed, not
//! merely available.
//!
//! ## How it's calculated
//!
//! ```text
//! Admission reconciliation rate = admissions reconciled / total admissions × 100
//!
//! Discharge reconciliation rate = discharges reconciled / total discharges × 100
//! ```
//!
//! ## Why it matters
//!
//! The Joint Commission lists medication reconciliation among its National
//! Patient Safety Goals precisely because transitions of care are where a
//! patient's medication list is most likely to silently diverge from what
//! they are actually taking. Admission and discharge reconciliation have
//! different completion barriers and different risk profiles — an
//! admission reconciliation error persists through the whole stay, while a
//! discharge reconciliation error follows the patient home — so the two
//! rates are tracked separately rather than blended into one.
//!
//! ## Worked example
//!
//! A hospital admits 1,000 patients in a month; reconciliation is completed
//! within 24 hours of admission for 850 of them. Of 950 discharges in the
//! same month, 760 have reconciliation completed before discharge.
//!
//! ```rust
//! use digital_health::medication_reconciliation_rate::{admission_reconciliation_rate, discharge_reconciliation_rate};
//!
//! // 850 / 1,000 × 100 = 85%.
//! let admission = admission_reconciliation_rate(850.0, 1_000.0).unwrap();
//! assert!((admission - 85.0).abs() < 1e-9);
//!
//! // 760 / 950 × 100 = 80%.
//! let discharge = discharge_reconciliation_rate(760.0, 950.0).unwrap();
//! assert!((discharge - 80.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The electronic health record's medication reconciliation module audit
//! log is the primary source, using its completion and sign-off timestamps
//! relative to the admission or discharge event.
//!
//! ## Pitfalls
//!
//! - **Counting a started-but-unsigned reconciliation as complete** — an
//!   in-progress reconciliation provides none of the safety benefit of a
//!   completed, reviewed one.
//! - **Blending admission and discharge rates into one figure** — the two
//!   have different completion barriers and different downstream risk if
//!   missed; report them separately.
//! - **A high rate driven by pro-forma sign-off** — a reconciliation
//!   completed without a genuine medication history review satisfies the
//!   metric without providing its safety benefit.
//!
//! ## Sources
//!
//! - The Joint Commission, National Patient Safety Goals on medication
//!   reconciliation.
//! - ONC / HealthIT.gov guidance on medication reconciliation and health IT
//!   safety.
//! - Peer-reviewed literature on medication reconciliation and adverse drug
//!   event reduction.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The admission reconciliation rate: admissions with a completed
/// medication reconciliation, as a percentage of total admissions.
///
/// # Arguments
///
/// * `admissions_reconciled` — admissions with a completed medication
///   reconciliation (count).
/// * `total_admissions` — total admissions in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_admissions` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::medication_reconciliation_rate::admission_reconciliation_rate;
///
/// // Worked example: 850 / 1,000 × 100 = 85%.
/// let rate = admission_reconciliation_rate(850.0, 1_000.0).unwrap();
/// assert!((rate - 85.0).abs() < 1e-9);
///
/// assert!(admission_reconciliation_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn admission_reconciliation_rate(
    admissions_reconciled: f64,
    total_admissions: f64,
) -> Option<f64> {
    if total_admissions == 0.0 {
        None
    } else {
        Some(admissions_reconciled / total_admissions * 100.0)
    }
}

/// The discharge reconciliation rate: discharges with a completed
/// medication reconciliation, as a percentage of total discharges.
///
/// # Arguments
///
/// * `discharges_reconciled` — discharges with a completed medication
///   reconciliation (count).
/// * `total_discharges` — total discharges in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_discharges` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::medication_reconciliation_rate::discharge_reconciliation_rate;
///
/// // Worked example: 760 / 950 × 100 = 80%.
/// let rate = discharge_reconciliation_rate(760.0, 950.0).unwrap();
/// assert!((rate - 80.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn discharge_reconciliation_rate(
    discharges_reconciled: f64,
    total_discharges: f64,
) -> Option<f64> {
    if total_discharges == 0.0 {
        None
    } else {
        Some(discharges_reconciled / total_discharges * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "reconciliation is completed within 24 hours of admission
    // for 850 of them" — 85%.
    #[test]
    fn worked_example_admission_rate_is_85_percent() {
        let rate = admission_reconciliation_rate(850.0, 1_000.0).unwrap();
        assert!((rate - 85.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "760 have reconciliation completed before discharge" — 80%.
    #[test]
    fn worked_example_discharge_rate_is_80_percent() {
        let rate = discharge_reconciliation_rate(760.0, 950.0).unwrap();
        assert!((rate - 80.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(admission_reconciliation_rate(1.0, 0.0).is_none());
        assert!(discharge_reconciliation_rate(1.0, 0.0).is_none());
    }
}

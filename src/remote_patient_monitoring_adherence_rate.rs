//! # Remote Patient Monitoring Adherence Rate
//!
//! Remote patient monitoring (RPM) adherence rate is the share of days in a
//! monitoring period on which a patient's connected device (blood-pressure
//! cuff, glucometer, pulse oximeter, weight scale) actually transmits a
//! qualifying reading. Unlike most engagement metrics in this crate, it has
//! a hard operational consequence: US billing codes for remote physiologic
//! and remote therapeutic monitoring require a minimum number of monitoring
//! days in the period before that period's device-supply code is billable
//! at all, so adherence below the threshold isn't just a weaker clinical
//! signal — it's a non-billable period.
//!
//! ## How it's calculated
//!
//! ```text
//! Monitoring adherence rate = days with a qualifying reading / days in period × 100
//!
//! Cohort billing-eligible rate = patients meeting the period's day threshold /
//!                                 total enrolled patients × 100
//! ```
//!
//! ## Why it matters
//!
//! CMS's remote physiologic monitoring codes (CPT 99453–99454) and remote
//! therapeutic monitoring codes (CPT 98975–98977) commonly require at least
//! 16 days of readings in a 30-day period for the device-supply code to be
//! billed for that period; other payers set their own thresholds. That
//! directly ties a clinical-engagement metric to program revenue, which
//! makes adherence a number both clinical and operational teams track.
//! Persistently low adherence concentrated in specific patient subgroups —
//! no home broadband, low digital literacy, limited device dexterity — is
//! also an equity signal, the same pattern seen in [patient portal adoption
//! rate](crate::patient_portal_adoption_rate).
//!
//! ## Worked example
//!
//! A cardiology remote monitoring program enrols 400 patients on a
//! connected blood-pressure cuff. One patient transmits a reading on 24 of
//! the 30 days in a billing period. Across the full cohort, 280 of the 400
//! enrolled patients meet the ≥16-day threshold for that period.
//!
//! ```rust
//! use digital_health::remote_patient_monitoring_adherence_rate::{
//!     monitoring_adherence_rate, billing_threshold_met_rate,
//! };
//!
//! // 24 / 30 × 100 = 80%.
//! let adherence = monitoring_adherence_rate(24.0, 30.0).unwrap();
//! assert!((adherence - 80.0).abs() < 1e-9);
//!
//! // 280 / 400 × 100 = 70% of the cohort clears the billing threshold.
//! let billing_eligible = billing_threshold_met_rate(280.0, 400.0).unwrap();
//! assert!((billing_eligible - 70.0).abs() < 1e-9);
//!
//! // A cohort can clear the billing threshold for most patients while still
//! // leaving meaningful room to improve individual adherence further.
//! assert!(billing_eligible < adherence);
//! ```
//!
//! ## Data sources and caveats
//!
//! The RPM/RTM platform's own device-transmission log is the primary
//! source: each device-day either has, or does not have, a qualifying
//! reading. Most platforms already compute, per patient and per period,
//! whether the billing-day threshold was met, which is the direct input to
//! `billing_threshold_met_rate`.
//!
//! ## Pitfalls
//!
//! - **Counting any transmission as qualifying** — a partial or
//!   out-of-range error reading can log a transmission event without being
//!   clinically usable; adherence measured on raw transmission counts
//!   overstates true engagement.
//! - **Ignoring the billing-window definition** — "16 of 30 days" is a
//!   calendar-period or rolling-period definition depending on payer;
//!   comparing adherence across payers without normalizing the window
//!   understates or overstates it.
//! - **Treating adherence as purely a patient-behaviour problem** — device
//!   reliability, cellular/Wi-Fi connectivity, and onboarding quality are
//!   provider- and vendor-side factors that materially affect it.
//!
//! ## Sources
//!
//! - Centers for Medicare & Medicaid Services (CMS), remote physiologic
//!   monitoring and remote therapeutic monitoring billing and coverage
//!   guidance (CPT 99453–99458, 98975–98981).
//! - Peer-reviewed literature on remote patient monitoring adherence and its
//!   association with digital-divide factors.
//! - Remote monitoring vendor and platform documentation on device-day
//!   transmission logging and billing-threshold reporting.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written; see Sources above.

/// The monitoring adherence rate: days with a qualifying reading as a
/// percentage of days in the monitoring period.
///
/// # Arguments
///
/// * `days_with_reading` — days in the period with at least one qualifying
///   device reading (count).
/// * `days_in_period` — total days in the monitoring period, commonly 30
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `days_in_period` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::remote_patient_monitoring_adherence_rate::monitoring_adherence_rate;
///
/// // Worked example: 24 / 30 × 100 = 80%.
/// let rate = monitoring_adherence_rate(24.0, 30.0).unwrap();
/// assert!((rate - 80.0).abs() < 1e-9);
///
/// assert!(monitoring_adherence_rate(24.0, 0.0).is_none());
/// ```
#[must_use]
pub fn monitoring_adherence_rate(days_with_reading: f64, days_in_period: f64) -> Option<f64> {
    if days_in_period == 0.0 {
        None
    } else {
        Some(days_with_reading / days_in_period * 100.0)
    }
}

/// The cohort billing-eligible rate: enrolled patients who met the period's
/// billing-day threshold, as a percentage of all enrolled patients.
///
/// # Arguments
///
/// * `patients_meeting_threshold` — enrolled patients whose monitoring days
///   met or exceeded the payer's threshold for the period (count).
/// * `total_enrolled_patients` — total patients enrolled in the program
///   during the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_enrolled_patients` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::remote_patient_monitoring_adherence_rate::billing_threshold_met_rate;
///
/// // Worked example: 280 / 400 × 100 = 70%.
/// let rate = billing_threshold_met_rate(280.0, 400.0).unwrap();
/// assert!((rate - 70.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn billing_threshold_met_rate(
    patients_meeting_threshold: f64,
    total_enrolled_patients: f64,
) -> Option<f64> {
    if total_enrolled_patients == 0.0 {
        None
    } else {
        Some(patients_meeting_threshold / total_enrolled_patients * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "One patient transmits a reading on 24 of the 30 days ...
    // 80%."
    #[test]
    fn worked_example_adherence_rate_is_80_percent() {
        let rate = monitoring_adherence_rate(24.0, 30.0).unwrap();
        assert!((rate - 80.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "280 of the 400 enrolled patients meet the ... threshold" —
    // a cohort billing-eligible rate of 70%.
    #[test]
    fn worked_example_billing_eligible_rate_is_70_percent() {
        let rate = billing_threshold_met_rate(280.0, 400.0).unwrap();
        assert!((rate - 70.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(monitoring_adherence_rate(1.0, 0.0).is_none());
        assert!(billing_threshold_met_rate(1.0, 0.0).is_none());
    }
}

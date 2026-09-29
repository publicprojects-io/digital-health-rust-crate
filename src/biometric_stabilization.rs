//! # Biometric Stabilization
//!
//! Biometric stabilization measures how many patients a connected device
//! keeps within a clinical target range: the share of monitored patients
//! whose blood pressure is controlled (under 130/80 mmHg via a cellular
//! cuff, for example), and the share of individual readings that fall in
//! range — for continuous glucose monitoring, time in range.
//!
//! ## How it's calculated
//!
//! ```text
//! Blood pressure control rate = patients controlled / patients monitored × 100
//!
//! Time in range = readings in range / readings taken × 100
//! ```
//!
//! ## Why it matters
//!
//! A single reading is noisy; stabilisation across a cohort and across time
//! is what predicts fewer complications. Control rate answers how many
//! patients reached target, and time in range answers how steadily each one
//! stayed there. It complements
//! [`biometric_improvement`](crate::biometric_improvement)'s change-from-
//! baseline view.
//!
//! ## Worked example
//!
//! A hypertension programme monitors 900 patients with cellular cuffs and
//! 540 are controlled below 130/80 mmHg at their latest review. Separately,
//! a CGM cohort takes 12,000 readings of which 8,400 are within 70–180
//! mg/dL.
//!
//! ```rust
//! use digital_health::biometric_stabilization::{blood_pressure_control_rate, time_in_range};
//!
//! // 540 / 900 × 100 = 60% controlled.
//! let blood_pressure_control_rate_value = blood_pressure_control_rate(540.0, 900.0).unwrap();
//! assert!((blood_pressure_control_rate_value - 60.0).abs() < 1e-9);
//!
//! // 8,400 / 12,000 × 100 = 70% in range.
//! let time_in_range_value = time_in_range(8_400.0, 12_000.0).unwrap();
//! assert!((time_in_range_value - 70.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Use the same control definition (for example, average of the last two
//! home readings) for every patient and period. Time in range requires
//! enough readings per patient; the CGM consensus asks for at least 70% of
//! possible readings over 14 days.
//!
//! ## Pitfalls
//!
//! - Counting patients with no recent readings as controlled or excluding
//!   them silently — say which.
//! - Using one clinic reading for control while quoting home-device data
//!   for time in range.
//! - Comparing time in range across targets — a tighter range gives a lower
//!   figure.
//!
//! ## Sources
//!
//! - Whelton, P. K. et al. 2017 ACC/AHA hypertension guideline, on the
//!   130/80 mmHg threshold.
//! - Battelino, T. et al. "Clinical Targets for Continuous Glucose
//!   Monitoring Data Interpretation." *Diabetes Care*, 2019.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The blood pressure control rate: patients with controlled blood pressure
/// as a percentage of patients monitored.
///
/// # Arguments
///
/// * `controlled` — monitored patients under the control threshold (count).
/// * `monitored` — patients with monitoring data in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `monitored` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::biometric_stabilization::blood_pressure_control_rate;
///
/// // Worked example: 540 / 900 × 100 = 60%.
/// let value = blood_pressure_control_rate(540.0, 900.0).unwrap();
/// assert!((value - 60.0).abs() < 1e-9);
///
/// assert!(blood_pressure_control_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn blood_pressure_control_rate(controlled: f64, monitored: f64) -> Option<f64> {
    if monitored == 0.0 {
        None
    } else {
        Some(controlled / monitored * 100.0)
    }
}

/// The time in range: readings within the target range as a percentage of
/// readings taken.
///
/// # Arguments
///
/// * `readings_in_range` — readings within the target range (count).
/// * `readings` — total readings taken (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `readings` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::biometric_stabilization::time_in_range;
///
/// // Worked example: 8,400 / 12,000 × 100 = 70%.
/// let value = time_in_range(8_400.0, 12_000.0).unwrap();
/// assert!((value - 70.0).abs() < 1e-9);
///
/// assert!(time_in_range(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn time_in_range(readings_in_range: f64, readings: f64) -> Option<f64> {
    if readings == 0.0 {
        None
    } else {
        Some(readings_in_range / readings * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "540 / 900 × 100 = 60% controlled."
    #[test]
    fn worked_example_blood_pressure_control_rate() {
        let value = blood_pressure_control_rate(540.0, 900.0).unwrap();
        assert!((value - 60.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "8,400 / 12,000 × 100 = 70% in range."
    #[test]
    fn worked_example_time_in_range() {
        let value = time_in_range(8_400.0, 12_000.0).unwrap();
        assert!((value - 70.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(blood_pressure_control_rate(1.0, 0.0).is_none());
        assert!(time_in_range(1.0, 0.0).is_none());
    }
}

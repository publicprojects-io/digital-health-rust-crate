//! # Digital Intake Form Completion Rate
//!
//! Digital intake form completion rate is the share of scheduled patients
//! who complete pre-visit registration and history forms online rather than
//! on paper at check-in, and the share of those online completions finished
//! *before* the patient arrives. It is a "digital front door" measure: the
//! first digital touchpoint most patients have with a visit, well before
//! [patient portal adoption rate](crate::patient_portal_adoption_rate)'s
//! own funnel begins.
//!
//! ## How it's calculated
//!
//! ```text
//! Intake completion rate = forms completed online / total scheduled patients × 100
//!
//! Pre-visit completion rate = forms completed before arrival / forms completed online × 100
//! ```
//!
//! ## Why it matters
//!
//! Online intake only saves front-desk time and shortens the visit when it
//! is completed *before* the patient arrives — a form finished on a tablet
//! in the waiting room still occupies staff time coordinating it and
//! provides none of the throughput benefit it's usually adopted for. The
//! two rates together separate "patients will use a digital form" from
//! "the digital form actually removed work from the visit."
//!
//! ## Worked example
//!
//! A clinic schedules 2,000 appointments in a month. 1,200 patients
//! complete their intake forms online; of those, 900 finish before
//! arriving.
//!
//! ```rust
//! use digital_health::digital_intake_form_completion_rate::{intake_completion_rate, pre_visit_completion_rate};
//!
//! // 1,200 / 2,000 × 100 = 60%.
//! let completion = intake_completion_rate(1_200.0, 2_000.0).unwrap();
//! assert!((completion - 60.0).abs() < 1e-9);
//!
//! // 900 / 1,200 × 100 = 75% finish before arriving.
//! let pre_visit = pre_visit_completion_rate(900.0, 1_200.0).unwrap();
//! assert!((pre_visit - 75.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The patient intake platform's own submission log is the primary source,
//! using its submission timestamp relative to the appointment's check-in
//! timestamp to separate pre-visit from at-arrival completion.
//!
//! ## Pitfalls
//!
//! - **Counting a partially completed form as completed** — an abandoned
//!   form that still requires front-desk follow-up provides none of the
//!   time savings a completed one does.
//! - **Not distinguishing pre-visit from at-arrival completion** — see "Why
//!   it matters" above; the blended completion rate alone can't tell them
//!   apart.
//! - **A form that doesn't sync structured data into the EHR** — data
//!   re-entered by staff from a "completed" digital form provides none of
//!   the double-entry reduction the tool is usually adopted for.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov, digital front-door and patient access guidance.
//! - Practice management literature on patient intake and registration
//!   digitization.
//! - Peer-reviewed literature on patient check-in technology and its effect
//!   on wait times.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The intake completion rate: forms completed online as a percentage of
/// total scheduled patients.
///
/// # Arguments
///
/// * `completed_online` — patients who completed intake forms online
///   (count).
/// * `total_scheduled` — total scheduled patients in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_intake_form_completion_rate::intake_completion_rate;
///
/// // Worked example: 1,200 / 2,000 × 100 = 60%.
/// let rate = intake_completion_rate(1_200.0, 2_000.0).unwrap();
/// assert!((rate - 60.0).abs() < 1e-9);
///
/// assert!(intake_completion_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn intake_completion_rate(completed_online: f64, total_scheduled: f64) -> Option<f64> {
    if total_scheduled == 0.0 {
        None
    } else {
        Some(completed_online / total_scheduled * 100.0)
    }
}

/// The pre-visit completion rate: online forms completed before arrival, as
/// a percentage of all forms completed online.
///
/// # Arguments
///
/// * `completed_before_arrival` — online forms completed before the patient
///   arrived (count).
/// * `completed_online` — total forms completed online in the period
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `completed_online` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_intake_form_completion_rate::pre_visit_completion_rate;
///
/// // Worked example: 900 / 1,200 × 100 = 75%.
/// let rate = pre_visit_completion_rate(900.0, 1_200.0).unwrap();
/// assert!((rate - 75.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn pre_visit_completion_rate(
    completed_before_arrival: f64,
    completed_online: f64,
) -> Option<f64> {
    if completed_online == 0.0 {
        None
    } else {
        Some(completed_before_arrival / completed_online * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "1,200 patients complete their intake forms online ... 60%."
    #[test]
    fn worked_example_intake_completion_rate_is_60_percent() {
        let rate = intake_completion_rate(1_200.0, 2_000.0).unwrap();
        assert!((rate - 60.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "900 finish before arriving ... 75%."
    #[test]
    fn worked_example_pre_visit_completion_rate_is_75_percent() {
        let rate = pre_visit_completion_rate(900.0, 1_200.0).unwrap();
        assert!((rate - 75.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(intake_completion_rate(1.0, 0.0).is_none());
        assert!(pre_visit_completion_rate(1.0, 0.0).is_none());
    }
}

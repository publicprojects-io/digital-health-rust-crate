//! # RE-AIM Framework
//!
//! The RE-AIM framework evaluates a digital intervention across five
//! dimensions: Reach, Effectiveness, Adoption, Implementation, and
//! Maintenance. Four are proportions this module computes — reach,
//! adoption, implementation fidelity, and maintenance. Effectiveness is an
//! outcome measure, and is calculated with the crate's outcome modules
//! instead, such as [`biometric_improvement`](crate::biometric_improvement)
//! or [`readmission_rate`](crate::readmission_rate).
//!
//! ## How it's calculated
//!
//! ```text
//! Reach = participants / eligible population × 100
//!
//! Adoption = settings adopting / settings invited × 100
//!
//! Implementation fidelity = components delivered as intended / components planned × 100
//!
//! Maintenance = participants still active at follow-up / initial participants × 100
//! ```
//!
//! ## Why it matters
//!
//! An intervention with a large effect in a trial can still fail in
//! practice because few eligible patients are reached, few clinics adopt
//! it, it is delivered inconsistently, or it fades. RE-AIM puts each of
//! those on its own line so a programme is judged on public-health impact,
//! not efficacy alone. Reach is also where digital equity shows up: compute
//! it per demographic group and compare it with [digital access
//! rate](crate::digital_access_rate).
//!
//! ## Worked example
//!
//! A diabetes-prevention app is offered to 6,000 eligible patients and
//! 2,400 enrol. 18 of the 30 invited clinics adopt it. Clinics deliver
//! 1,350 of 1,500 planned programme components as intended. Six months on,
//! 1,080 of the 2,400 enrolled patients are still active.
//!
//! ```rust
//! use digital_health::re_aim_framework::{reach_rate, adoption_rate, implementation_fidelity_rate, maintenance_rate};
//!
//! // 2,400 / 6,000 × 100 = 40% reach.
//! let reach_rate_value = reach_rate(2_400.0, 6_000.0).unwrap();
//! assert!((reach_rate_value - 40.0).abs() < 1e-9);
//!
//! // 18 / 30 × 100 = 60% adoption.
//! let adoption_rate_value = adoption_rate(18.0, 30.0).unwrap();
//! assert!((adoption_rate_value - 60.0).abs() < 1e-9);
//!
//! // 1,350 / 1,500 × 100 = 90% fidelity.
//! let implementation_fidelity_rate_value = implementation_fidelity_rate(1_350.0, 1_500.0).unwrap();
//! assert!((implementation_fidelity_rate_value - 90.0).abs() < 1e-9);
//!
//! // 1,080 / 2,400 × 100 = 45% maintained.
//! let maintenance_rate_value = maintenance_rate(1_080.0, 2_400.0).unwrap();
//! assert!((maintenance_rate_value - 45.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Fix the numerator and denominator definition for each dimension before
//! the evaluation begins: who is eligible, what counts as an adopting
//! setting, what the intended delivery is, and the follow-up point for
//! maintenance. Report each dimension for subgroups as well as overall.
//!
//! ## Pitfalls
//!
//! - Reporting only Effectiveness — a strong effect reaching 5% of eligible
//!   patients has small population impact.
//! - Using enrolled patients rather than the eligible population as the
//!   reach denominator.
//! - Treating adoption at the setting level as if it were patient-level
//!   reach.
//!
//! ## Sources
//!
//! - Glasgow, R. E., Vogt, T. M. and Boles, S. M. "Evaluating the Public
//!   Health Impact of Health Promotion Interventions: The RE-AIM
//!   Framework." *American Journal of Public Health*, 1999.
//! - RE-AIM.org planning and evaluation guidance.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The reach rate: participants as a percentage of the eligible population.
///
/// # Arguments
///
/// * `participants` — eligible people who took part (count).
/// * `eligible` — people eligible to take part (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `eligible` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::re_aim_framework::reach_rate;
///
/// // Worked example: 2,400 / 6,000 × 100 = 40%.
/// let value = reach_rate(2_400.0, 6_000.0).unwrap();
/// assert!((value - 40.0).abs() < 1e-9);
///
/// assert!(reach_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn reach_rate(participants: f64, eligible: f64) -> Option<f64> {
    if eligible == 0.0 {
        None
    } else {
        Some(participants / eligible * 100.0)
    }
}

/// The adoption rate: settings that adopted the intervention as a
/// percentage of settings invited.
///
/// # Arguments
///
/// * `adopting_settings` — clinics or sites that adopted it (count).
/// * `invited_settings` — clinics or sites invited (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `invited_settings` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::re_aim_framework::adoption_rate;
///
/// // Worked example: 18 / 30 × 100 = 60%.
/// let value = adoption_rate(18.0, 30.0).unwrap();
/// assert!((value - 60.0).abs() < 1e-9);
///
/// assert!(adoption_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn adoption_rate(adopting_settings: f64, invited_settings: f64) -> Option<f64> {
    if invited_settings == 0.0 {
        None
    } else {
        Some(adopting_settings / invited_settings * 100.0)
    }
}

/// The implementation fidelity rate: components delivered as intended as a
/// percentage of components planned.
///
/// # Arguments
///
/// * `delivered_as_intended` — components delivered per protocol (count).
/// * `planned` — components planned (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `planned` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::re_aim_framework::implementation_fidelity_rate;
///
/// // Worked example: 1,350 / 1,500 × 100 = 90%.
/// let value = implementation_fidelity_rate(1_350.0, 1_500.0).unwrap();
/// assert!((value - 90.0).abs() < 1e-9);
///
/// assert!(implementation_fidelity_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn implementation_fidelity_rate(delivered_as_intended: f64, planned: f64) -> Option<f64> {
    if planned == 0.0 {
        None
    } else {
        Some(delivered_as_intended / planned * 100.0)
    }
}

/// The maintenance rate: participants still active at the follow-up point
/// as a percentage of initial participants.
///
/// # Arguments
///
/// * `still_active` — participants active at follow-up (count).
/// * `initial_participants` — participants at the start (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `initial_participants` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::re_aim_framework::maintenance_rate;
///
/// // Worked example: 1,080 / 2,400 × 100 = 45%.
/// let value = maintenance_rate(1_080.0, 2_400.0).unwrap();
/// assert!((value - 45.0).abs() < 1e-9);
///
/// assert!(maintenance_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn maintenance_rate(still_active: f64, initial_participants: f64) -> Option<f64> {
    if initial_participants == 0.0 {
        None
    } else {
        Some(still_active / initial_participants * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "2,400 / 6,000 × 100 = 40% reach."
    #[test]
    fn worked_example_reach_rate() {
        let value = reach_rate(2_400.0, 6_000.0).unwrap();
        assert!((value - 40.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "18 / 30 × 100 = 60% adoption."
    #[test]
    fn worked_example_adoption_rate() {
        let value = adoption_rate(18.0, 30.0).unwrap();
        assert!((value - 60.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "1,350 / 1,500 × 100 = 90% fidelity."
    #[test]
    fn worked_example_implementation_fidelity_rate() {
        let value = implementation_fidelity_rate(1_350.0, 1_500.0).unwrap();
        assert!((value - 90.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "1,080 / 2,400 × 100 = 45% maintained."
    #[test]
    fn worked_example_maintenance_rate() {
        let value = maintenance_rate(1_080.0, 2_400.0).unwrap();
        assert!((value - 45.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(reach_rate(1.0, 0.0).is_none());
        assert!(adoption_rate(1.0, 0.0).is_none());
        assert!(implementation_fidelity_rate(1.0, 0.0).is_none());
        assert!(maintenance_rate(1.0, 0.0).is_none());
    }
}

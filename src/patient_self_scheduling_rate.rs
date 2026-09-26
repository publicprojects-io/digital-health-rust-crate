//! # Patient Self-Scheduling Rate
//!
//! Patient self-scheduling rate is the share of appointments a patient books
//! directly online — through a patient portal or app — rather than by phone
//! through staff, and the share of those self-scheduled appointments booked
//! for same-day or next-day access. It is the natural continuation of
//! [patient portal adoption rate](crate::patient_portal_adoption_rate)'s
//! funnel: a patient who has activated a portal account but never
//! self-schedules hasn't realized this specific piece of the portal's
//! value, and online self-scheduling is one of the "best-evidenced levers"
//! [appointment no-show rate](crate::appointment_no_show_rate) itself
//! points to for reducing no-shows.
//!
//! ## How it's calculated
//!
//! ```text
//! Self-scheduling rate = self-scheduled appointments / total scheduled appointments × 100
//!
//! Same-day self-scheduling rate = same-day/next-day self-scheduled appointments /
//!                                  self-scheduled appointments × 100
//! ```
//!
//! ## Why it matters
//!
//! Adoption of self-scheduling alone doesn't guarantee the access
//! improvement it's usually deployed to achieve — a system that only
//! surfaces appointments weeks out has not solved the same problem as one
//! that offers genuine same-day access, even at an identical
//! self-scheduling rate. The two numbers together separate "patients are
//! using the tool" from "the tool is actually improving access," the same
//! distinction [patient portal adoption
//! rate](crate::patient_portal_adoption_rate) draws between registration and
//! active use.
//!
//! ## Worked example
//!
//! Continuing [appointment no-show rate](crate::appointment_no_show_rate)'s
//! worked example: a community clinic schedules 2,000 appointments in a
//! month. 640 are booked by the patient directly online; of those, 192 are
//! for same-day or next-day availability.
//!
//! ```rust
//! use digital_health::patient_self_scheduling_rate::{self_scheduling_rate, same_day_self_scheduling_rate};
//!
//! // 640 / 2,000 × 100 = 32%.
//! let self_scheduled = self_scheduling_rate(640.0, 2_000.0).unwrap();
//! assert!((self_scheduled - 32.0).abs() < 1e-9);
//!
//! // 192 / 640 × 100 = 30% of self-scheduled visits are same-day or next-day.
//! let same_day = same_day_self_scheduling_rate(192.0, 640.0).unwrap();
//! assert!((same_day - 30.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The practice management or scheduling system's booking log is the
//! primary source, using its booking-channel field; the field must reflect
//! who actually initiated the transaction (patient online, versus staff
//! over the phone even while looking at the same online calendar), not
//! which calendar was displayed during the call.
//!
//! ## Pitfalls
//!
//! - **Counting staff-assisted bookings as self-scheduled** — a staff
//!   member viewing the online calendar while booking by phone is not the
//!   same transaction as the patient booking it themselves; the two must be
//!   distinguished in the booking-channel field.
//! - **Reporting adoption without same-day access** — a high self-scheduling
//!   rate with no near-term availability has automated the booking
//!   transaction without improving the access it's usually justified by.
//! - **Not segmenting by visit type** — a system enabled only for routine
//!   follow-ups puts a structural ceiling on this rate regardless of
//!   patient willingness to use it.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov, digital front-door and patient access guidance.
//! - Peer-reviewed literature on online scheduling adoption and its
//!   relationship to no-show rates.
//! - Healthcare operations literature on same-day ("advanced access" /
//!   "open access") scheduling models.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The self-scheduling rate: appointments booked by the patient online, as a
/// percentage of total scheduled appointments.
///
/// # Arguments
///
/// * `self_scheduled` — appointments booked by the patient directly online
///   (count).
/// * `total_scheduled` — total scheduled appointments in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_self_scheduling_rate::self_scheduling_rate;
///
/// // Worked example: 640 / 2,000 × 100 = 32%.
/// let rate = self_scheduling_rate(640.0, 2_000.0).unwrap();
/// assert!((rate - 32.0).abs() < 1e-9);
///
/// assert!(self_scheduling_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn self_scheduling_rate(self_scheduled: f64, total_scheduled: f64) -> Option<f64> {
    if total_scheduled == 0.0 {
        None
    } else {
        Some(self_scheduled / total_scheduled * 100.0)
    }
}

/// The same-day self-scheduling rate: self-scheduled appointments booked for
/// same-day or next-day availability, as a percentage of all self-scheduled
/// appointments.
///
/// # Arguments
///
/// * `same_day_self_scheduled` — self-scheduled appointments booked for
///   same-day or next-day availability (count).
/// * `self_scheduled` — total self-scheduled appointments in the period
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `self_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_self_scheduling_rate::same_day_self_scheduling_rate;
///
/// // Worked example: 192 / 640 × 100 = 30%.
/// let rate = same_day_self_scheduling_rate(192.0, 640.0).unwrap();
/// assert!((rate - 30.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn same_day_self_scheduling_rate(
    same_day_self_scheduled: f64,
    self_scheduled: f64,
) -> Option<f64> {
    if self_scheduled == 0.0 {
        None
    } else {
        Some(same_day_self_scheduled / self_scheduled * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "640 are booked by the patient directly online ... 32%."
    #[test]
    fn worked_example_self_scheduling_rate_is_32_percent() {
        let rate = self_scheduling_rate(640.0, 2_000.0).unwrap();
        assert!((rate - 32.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "192 are for same-day or next-day availability ... 30%."
    #[test]
    fn worked_example_same_day_rate_is_30_percent() {
        let rate = same_day_self_scheduling_rate(192.0, 640.0).unwrap();
        assert!((rate - 30.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(self_scheduling_rate(1.0, 0.0).is_none());
        assert!(same_day_self_scheduling_rate(1.0, 0.0).is_none());
    }
}

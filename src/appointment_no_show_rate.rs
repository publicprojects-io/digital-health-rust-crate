//! # Appointment No-Show Rate
//!
//! Appointment no-show rate (also called "did not attend", or DNA, rate) is
//! the share of scheduled appointments where the patient neither attended
//! nor cancelled with reasonable notice. It is one of the oldest operational
//! metrics in healthcare, and digital tools — reminders, self-service
//! rescheduling, and portal-based booking — are now among the most
//! effective and best-evidenced levers for reducing it.
//!
//! ## How it's calculated
//!
//! ```text
//! No-show rate = appointments marked "did not attend" / total scheduled appointments × 100
//! ```
//!
//! An appointment cancelled by either party with more than a defined notice
//! period (commonly 24 hours) is typically excluded from the denominator, or
//! moved to a separate category. Late cancellations (below that notice
//! period) are usually reported separately from true no-shows, since the
//! operational and behavioural implications differ.
//!
//! ## Why it matters
//!
//! Every no-show is a unit of clinical capacity that cannot usually be
//! recovered, so the rate directly drives waiting-list length, cost per
//! completed appointment, and clinician time lost. No-show behaviour
//! correlates with deprivation, transport access, and the burden of
//! managing multiple long-term conditions, so treating a high rate purely as
//! a patient behaviour problem tends to produce interventions (such as
//! blanket penalties) that entrench inequity rather than reduce it. Digital
//! reminders and easy digital rescheduling are consistently among the most
//! effective, low-cost interventions available.
//!
//! ## Worked example
//!
//! A community clinic schedules 2,000 appointments in a month. Of these, 140
//! are cancelled with more than 24 hours' notice (rebooked and excluded from
//! the denominator), 60 are cancelled late (under 24 hours), and 180 are
//! recorded as a true no-show with no contact at all.
//!
//! ```rust
//! use digital_health::appointment_no_show_rate::{no_show_rate, combined_non_attendance_rate};
//!
//! // 180 / 2,000 × 100 = 9%.
//! let rate = no_show_rate(180.0, 2_000.0).unwrap();
//! assert!((rate - 9.0).abs() < 1e-9);
//!
//! // Folding the 60 late cancellations into the same bucket raises it to 12% —
//! // which is why the definition used must always be stated alongside the figure.
//! let combined = combined_non_attendance_rate(180.0, 60.0, 2_000.0).unwrap();
//! assert!((combined - 12.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The scheduling or practice management system is the primary source,
//! using its appointment status codes; the metric's quality depends
//! entirely on staff consistently using the correct status rather than a
//! generic "cancelled" bucket for everything.
//!
//! ## Pitfalls
//!
//! - **Comparing raw rates across clinics with different overbooking
//!   practices** — a clinic that deliberately overbooks to compensate for an
//!   expected no-show rate will show a different apparent rate than one that
//!   does not, independent of true patient behaviour.
//! - **Conflating late cancellations with true no-shows** — the two have
//!   different causes and different digital fixes.
//! - **Survivorship bias from discharge policies** — services that discharge
//!   patients after repeated no-shows will see their own rate improve
//!   mechanically, while simply displacing the same patients elsewhere.
//! - **Blaming digital exclusion on the patient** — a multi-channel approach
//!   (letter, call, text, app) is usually needed to avoid widening access
//!   gaps.
//!
//! ## Sources
//!
//! - NHS England, missed appointments in general practice and outpatient
//!   care, published statistics and guidance.
//! - Cochrane systematic reviews on interventions to reduce missed
//!   healthcare appointments, including reminder systems.
//! - Peer-reviewed literature on socioeconomic and demographic correlates of
//!   appointment non-attendance.
//!
//! Topic doc: digital-health-metrics/locales/en-gb-oxendict/topics/appointment-no-show-rate/index.md

/// The no-show rate: true no-shows as a percentage of total scheduled
/// appointments.
///
/// # Arguments
///
/// * `no_shows` — appointments recorded as "did not attend" (count).
/// * `total_scheduled` — total scheduled appointments in the period, after
///   excluding appointments cancelled with sufficient notice (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::appointment_no_show_rate::no_show_rate;
///
/// // Worked example: 180 / 2,000 × 100 = 9%.
/// let rate = no_show_rate(180.0, 2_000.0).unwrap();
/// assert!((rate - 9.0).abs() < 1e-9);
///
/// assert!(no_show_rate(180.0, 0.0).is_none());
/// ```
pub fn no_show_rate(no_shows: f64, total_scheduled: f64) -> Option<f64> {
    if total_scheduled == 0.0 {
        None
    } else {
        Some(no_shows / total_scheduled * 100.0)
    }
}

/// The late-cancellation rate: appointments cancelled below the notice
/// threshold as a percentage of total scheduled appointments.
///
/// # Arguments
///
/// * `late_cancellations` — appointments cancelled below the notice
///   threshold, commonly 24 hours (count).
/// * `total_scheduled` — total scheduled appointments in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::appointment_no_show_rate::late_cancellation_rate;
///
/// // Worked example: 60 / 2,000 × 100 = 3%.
/// let rate = late_cancellation_rate(60.0, 2_000.0).unwrap();
/// assert!((rate - 3.0).abs() < 1e-9);
/// ```
pub fn late_cancellation_rate(late_cancellations: f64, total_scheduled: f64) -> Option<f64> {
    if total_scheduled == 0.0 {
        None
    } else {
        Some(late_cancellations / total_scheduled * 100.0)
    }
}

/// The combined non-attendance rate: true no-shows plus late cancellations,
/// as a single percentage of total scheduled appointments.
///
/// Folding late cancellations into the same bucket as true no-shows raises
/// the reported rate — the doc's worked example goes from 9% to 12% — which
/// is why the definition used must always be stated alongside the figure.
///
/// # Arguments
///
/// * `no_shows` — appointments recorded as "did not attend" (count).
/// * `late_cancellations` — appointments cancelled below the notice
///   threshold (count).
/// * `total_scheduled` — total scheduled appointments in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_scheduled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::appointment_no_show_rate::combined_non_attendance_rate;
///
/// // Worked example: (180 + 60) / 2,000 × 100 = 12%.
/// let rate = combined_non_attendance_rate(180.0, 60.0, 2_000.0).unwrap();
/// assert!((rate - 12.0).abs() < 1e-9);
/// ```
pub fn combined_non_attendance_rate(
    no_shows: f64,
    late_cancellations: f64,
    total_scheduled: f64,
) -> Option<f64> {
    if total_scheduled == 0.0 {
        None
    } else {
        Some((no_shows + late_cancellations) / total_scheduled * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "The no-show rate is 180 / 2,000 × 100 = 9%."
    #[test]
    fn worked_example_no_show_rate_is_9_percent() {
        let rate = no_show_rate(180.0, 2_000.0).unwrap();
        assert!((rate - 9.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "If the 60 late cancellations were folded into the same
    // category as true no-shows, the reported rate would rise to 12%."
    #[test]
    fn worked_example_combined_rate_is_12_percent() {
        let rate = combined_non_attendance_rate(180.0, 60.0, 2_000.0).unwrap();
        assert!((rate - 12.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(no_show_rate(1.0, 0.0).is_none());
        assert!(late_cancellation_rate(1.0, 0.0).is_none());
        assert!(combined_non_attendance_rate(1.0, 1.0, 0.0).is_none());
    }
}

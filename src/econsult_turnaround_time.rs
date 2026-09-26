//! # E-Consult Turnaround Time
//!
//! E-consult turnaround time is the elapsed time from a primary care
//! clinician submitting an asynchronous electronic consultation (e-consult)
//! to a specialist's written response, for a clinical question that does
//! not require a full referral visit. It is the operational measure of
//! whether an e-consult service is fast enough to actually change referral
//! behaviour, the same role [digital referral turnaround
//! time](crate::digital_referral_turnaround_time) plays for full referrals.
//!
//! ## How it's calculated
//!
//! ```text
//! Turnaround time = timestamp(specialist response) − timestamp(e-consult submission)
//!
//! Report median and a high percentile (commonly the 90th), for the same
//! reason as digital referral turnaround time: the distribution is
//! right-skewed by complex or returned questions.
//! ```
//!
//! ## Why it matters
//!
//! Programmes such as the Champlain BASE eConsult service (and similar
//! asynchronous specialist-advice models) are widely studied specifically
//! because they let primary care resolve many specialty questions without
//! the wait, cost, and travel of a full in-person referral — but only if
//! the response comes back fast enough to be useful within the primary
//! care visit's own timeline. Turnaround time is the number that decides
//! whether that promise is being kept.
//!
//! ## Worked example
//!
//! An e-consult service logs specialist response times (in hours) for ten
//! e-consults in a month: 1, 2, 3, 4, 5, 5, 8, 12, 48, and 48 hours.
//!
//! ```rust
//! use digital_health::econsult_turnaround_time::{elapsed_hours, percentile};
//!
//! // A single e-consult: submitted at hour 100.0, answered at hour 104.0.
//! let elapsed = elapsed_hours(100.0, 104.0);
//! assert!((elapsed - 4.0).abs() < 1e-9);
//!
//! let times = [1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 8.0, 12.0, 48.0, 48.0];
//! let median = percentile(&times, 50.0).unwrap();
//! let p90 = percentile(&times, 90.0).unwrap();
//! assert!((median - 5.0).abs() < 1e-9);
//! assert!((p90 - 48.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The e-consult platform's own submission and response timestamps are the
//! primary source. Whether the "clock" pauses while a specialist requests
//! more information from the referrer should be confirmed and stated
//! alongside the figure, the same caution [digital referral turnaround
//! time](crate::digital_referral_turnaround_time) raises for its own clock
//! convention.
//!
//! ## Pitfalls
//!
//! - **Reporting the mean instead of the median and percentiles** — a small
//!   number of long-running e-consults will pull the mean far above the
//!   typical response time.
//! - **Not distinguishing "resolved without a referral" from "recommended a
//!   referral anyway"** — a fast turnaround time on an e-consult that ends
//!   in a referral regardless is not the same operational win as one that
//!   avoided the referral entirely; turnaround time alone can't tell them
//!   apart.
//! - **Comparing turnaround times across specialties without segmenting** —
//!   response time norms vary enormously by specialty and question
//!   complexity.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on eConsult services, including the Champlain
//!   BASE eConsult service and similar asynchronous specialist-advice
//!   programmes.
//! - ONC / HealthIT.gov guidance on asynchronous consultation and
//!   interoperability.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// Elapsed time between an e-consult's submission and the specialist's
/// response.
///
/// # Arguments
///
/// * `submission_hour` — the e-consult's submission timestamp, expressed as
///   an hour number on any consistent clock (e.g. hours since an epoch).
/// * `response_hour` — the specialist's response timestamp, on the same
///   clock.
///
/// # Returns
///
/// Elapsed time in hours: `response_hour − submission_hour`.
///
/// # Examples
///
/// ```rust
/// use digital_health::econsult_turnaround_time::elapsed_hours;
///
/// let elapsed = elapsed_hours(100.0, 104.0);
/// assert!((elapsed - 4.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn elapsed_hours(submission_hour: f64, response_hour: f64) -> f64 {
    response_hour - submission_hour
}

/// Empirical percentile (0–100) of a set of e-consult turnaround times, by
/// linear interpolation between order statistics.
///
/// # Arguments
///
/// * `turnaround_times` — turnaround times, in any consistent unit (unsorted
///   is fine; a sorted copy is made).
/// * `p` — percentile in `[0, 100]`, e.g. `50.0` for the median, `90.0` for
///   the 90th percentile. Out-of-range values are clamped.
///
/// # Returns
///
/// `Some(interpolated value)`, or `None` for an empty set.
///
/// # Examples
///
/// ```rust
/// use digital_health::econsult_turnaround_time::percentile;
///
/// let times = [1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 8.0, 12.0, 48.0, 48.0];
/// assert_eq!(percentile(&times, 50.0), Some(5.0));
/// assert_eq!(percentile(&times, 90.0), Some(48.0));
/// assert!(percentile(&[], 50.0).is_none());
/// ```
#[must_use]
pub fn percentile(turnaround_times: &[f64], p: f64) -> Option<f64> {
    crate::internal::percentile(turnaround_times, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "submitted at hour 100.0, answered at hour 104.0."
    #[test]
    fn worked_example_elapsed_hours_is_4() {
        let elapsed = elapsed_hours(100.0, 104.0);
        assert!((elapsed - 4.0).abs() < 1e-9, "got {elapsed}");
    }

    // Doc line: response times "1, 2, 3, 4, 5, 5, 8, 12, 48, and 48 hours."
    #[test]
    fn worked_example_median_and_p90_match_doc() {
        let times = [1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 8.0, 12.0, 48.0, 48.0];
        let median = percentile(&times, 50.0).unwrap();
        let p90 = percentile(&times, 90.0).unwrap();
        assert!((median - 5.0).abs() < 1e-9, "got median {median}");
        assert!((p90 - 48.0).abs() < 1e-9, "got p90 {p90}");
    }

    #[test]
    fn empty_turnaround_times_returns_none() {
        assert!(percentile(&[], 50.0).is_none());
    }
}

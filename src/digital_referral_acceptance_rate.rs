//! # Digital Referral Acceptance Rate
//!
//! Digital referral acceptance rate is the share of triaged e-referrals
//! accepted and booked by the receiving service, and the share returned to
//! the referrer for missing information rather than accepted or rejected
//! outright. [Digital referral turnaround time](crate::digital_referral_turnaround_time)
//! measures how fast triage happens; this module measures what triage
//! actually decides.
//!
//! ## How it's calculated
//!
//! ```text
//! Acceptance rate = referrals accepted / total referrals triaged × 100
//!
//! Returned-for-information rate = referrals returned for missing information /
//!                                  total referrals triaged × 100
//! ```
//!
//! ## Why it matters
//!
//! A fast turnaround time on a referral that just gets returned for missing
//! information is not the same operational win as a fast acceptance — and a
//! rising returned-for-information rate is precisely the rework loop
//! [digital referral turnaround
//! time](crate::digital_referral_turnaround_time)'s own rustdoc warns is
//! easy to miss if only measuring referrals that pass through cleanly. A
//! service reporting an improving turnaround time should check whether that
//! improvement is being driven by faster genuine accept/reject decisions or
//! by a larger share of referrals being returned quickly instead.
//!
//! ## Worked example
//!
//! An e-referral system triages 1,000 referrals in a month. 780 are
//! accepted and booked; 150 are returned to the referrer for missing
//! information; the remainder are rejected as clinically inappropriate.
//!
//! ```rust
//! use digital_health::digital_referral_acceptance_rate::{acceptance_rate, returned_for_information_rate};
//!
//! // 780 / 1,000 × 100 = 78%.
//! let accepted = acceptance_rate(780.0, 1_000.0).unwrap();
//! assert!((accepted - 78.0).abs() < 1e-9);
//!
//! // 150 / 1,000 × 100 = 15%.
//! let returned = returned_for_information_rate(150.0, 1_000.0).unwrap();
//! assert!((returned - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The e-referral or referral management system's own triage-outcome audit
//! log is the primary source, using its decision field (accepted, returned,
//! rejected).
//!
//! ## Pitfalls
//!
//! - **Reading turnaround time and acceptance rate independently** — a
//!   fast-looking turnaround time is often driven by returns, which are
//!   typically triaged faster than genuine accept/reject decisions.
//! - **Not distinguishing "returned for missing information" from
//!   "rejected as clinically inappropriate"** — the two require completely
//!   different referrer-side interventions: a template or form fix versus
//!   referral-criteria education.
//! - **Comparing acceptance rates across specialties without segmenting** —
//!   baseline acceptance rates vary substantially by specialty and referral
//!   pathway design.
//!
//! ## Sources
//!
//! - NHS England, e-Referral Service (e-RS) statistics and specifications —
//!   the same evidence base as [digital referral turnaround
//!   time](crate::digital_referral_turnaround_time).
//! - Peer-reviewed literature on electronic referral management and
//!   return-to-referrer rework.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The acceptance rate: referrals accepted and booked, as a percentage of
/// total referrals triaged.
///
/// # Arguments
///
/// * `accepted` — referrals accepted and booked by the receiving service
///   (count).
/// * `total_triaged` — total referrals triaged in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_triaged` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_referral_acceptance_rate::acceptance_rate;
///
/// // Worked example: 780 / 1,000 × 100 = 78%.
/// let rate = acceptance_rate(780.0, 1_000.0).unwrap();
/// assert!((rate - 78.0).abs() < 1e-9);
///
/// assert!(acceptance_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn acceptance_rate(accepted: f64, total_triaged: f64) -> Option<f64> {
    if total_triaged == 0.0 {
        None
    } else {
        Some(accepted / total_triaged * 100.0)
    }
}

/// The returned-for-information rate: referrals returned to the referrer
/// for missing information, as a percentage of total referrals triaged.
///
/// # Arguments
///
/// * `returned` — referrals returned to the referrer for missing
///   information (count).
/// * `total_triaged` — total referrals triaged in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_triaged` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_referral_acceptance_rate::returned_for_information_rate;
///
/// // Worked example: 150 / 1,000 × 100 = 15%.
/// let rate = returned_for_information_rate(150.0, 1_000.0).unwrap();
/// assert!((rate - 15.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn returned_for_information_rate(returned: f64, total_triaged: f64) -> Option<f64> {
    if total_triaged == 0.0 {
        None
    } else {
        Some(returned / total_triaged * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "780 are accepted and booked" out of 1,000 — 78%.
    #[test]
    fn worked_example_acceptance_rate_is_78_percent() {
        let rate = acceptance_rate(780.0, 1_000.0).unwrap();
        assert!((rate - 78.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "150 are returned to the referrer for missing information"
    // — 15%.
    #[test]
    fn worked_example_returned_rate_is_15_percent() {
        let rate = returned_for_information_rate(150.0, 1_000.0).unwrap();
        assert!((rate - 15.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(acceptance_rate(1.0, 0.0).is_none());
        assert!(returned_for_information_rate(1.0, 0.0).is_none());
    }
}

//! # Digital Referral Turnaround Time
//!
//! Digital referral turnaround time is the elapsed time from an electronic
//! referral being submitted by a referring clinician to it being triaged and
//! either accepted, rejected, or booked by the receiving service. It is a
//! process (flow) metric, distinct from total patient waiting time, and it
//! is one of the clearest places where a digital system change (structured
//! e-referral, image-based triage, standardized referral forms) can be shown
//! to move an operational number rather than merely a satisfaction score.
//!
//! ## How it's calculated
//!
//! ```text
//! Turnaround time = timestamp(triage decision) − timestamp(referral submission)
//!
//! Report median and a high percentile (commonly the 90th), not only the
//! mean, because the distribution is heavily right-skewed by returned or
//! complex referrals.
//! ```
//!
//! ## Why it matters
//!
//! A slow or highly variable triage step adds delay before a patient even
//! joins a clinical waiting list, and because that delay happens before any
//! clinical care starts, it is pure process waste that digital tooling is
//! well placed to remove. Referral systems that force a "return to referrer"
//! cycle for missing information create rework loops that are easy to miss
//! if turnaround time is only measured on referrals that pass through
//! cleanly the first time.
//!
//! ## Worked example
//!
//! An e-referral system's audit trail shows a median time from submission to
//! triage decision of 1.8 days across all specialties, with a 90th-percentile
//! time of 6 days, driven mainly by referrals returned for missing clinical
//! information. A teledermatology pathway using image-based triage on the
//! same platform achieves a median turnaround of 4 hours.
//!
//! ```rust
//! use digital_health::digital_referral_turnaround_time::{elapsed_days, hours_to_days, percentile};
//!
//! // Elapsed time is simply the decision timestamp minus the submission timestamp.
//! let elapsed = elapsed_days(10.0, 11.8);
//! assert!((elapsed - 1.8).abs() < 1e-9);
//!
//! // A month of turnaround times (days), right-skewed by returned referrals —
//! // illustrative of the doc's stated median 1.8 days and 90th percentile 6 days.
//! let durations = [0.3, 0.5, 0.8, 1.2, 1.8, 1.8, 2.5, 3.5, 6.0, 6.0];
//! let median = percentile(&durations, 50.0).unwrap();
//! let p90 = percentile(&durations, 90.0).unwrap();
//! assert!((median - 1.8).abs() < 1e-9);
//! assert!((p90 - 6.0).abs() < 1e-9);
//!
//! // Teledermatology's 4-hour median, expressed in days.
//! let teledermatology_median = hours_to_days(4.0);
//! assert!((teledermatology_median - (4.0 / 24.0)).abs() < 1e-9);
//! assert!(teledermatology_median < median);
//! ```
//!
//! ## Data sources and caveats
//!
//! The e-referral or referral management system's own audit trail is the
//! primary source, using submission and decision timestamps; organizations
//! should confirm whether the "clock" pauses while a referral is returned
//! for more information or runs continuously, since the two definitions
//! produce materially different figures for the same underlying process.
//!
//! ## Pitfalls
//!
//! - **Measuring only "clean" referrals** — excluding rejected or returned
//!   referrals hides the rework burden that digital tooling is often
//!   specifically meant to reduce.
//! - **Reporting the mean instead of the median and percentiles** — a small
//!   number of long-running, returned referrals will pull the mean far above
//!   the typical patient's actual experience.
//! - **Confusing turnaround time with total wait time** — turnaround time
//!   covers only the triage step; the patient's total experience also
//!   includes the downstream clinical waiting list.
//! - **Not distinguishing sub-stages** — a service that only measures
//!   end-to-end time cannot tell whether a slow figure is caused by
//!   referrers, triage capacity, or both.
//!
//! ## Sources
//!
//! - NHS England, e-Referral Service (e-RS) statistics and service
//!   specifications.
//! - Peer-reviewed literature on electronic referral management systems and
//!   digital triage pathways, including teledermatology.
//! - ONC / HealthIT.gov, interoperability and referral coordination
//!   guidance.
//!
//! Topic doc: digital-health-metrics/locales/en-gb-oxendict/topics/digital-referral-turnaround-time/index.md

/// Elapsed time between a referral's submission and its triage decision.
///
/// # Arguments
///
/// * `submission_day` — the referral's submission timestamp, expressed as a
///   day number on any consistent clock (e.g. days since an epoch).
/// * `decision_day` — the triage decision's timestamp, on the same clock.
///
/// # Returns
///
/// Elapsed time in days: `decision_day − submission_day`.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_referral_turnaround_time::elapsed_days;
///
/// let elapsed = elapsed_days(10.0, 11.8);
/// assert!((elapsed - 1.8).abs() < 1e-9);
/// ```
#[must_use]
pub fn elapsed_days(submission_day: f64, decision_day: f64) -> f64 {
    decision_day - submission_day
}

/// Converts an elapsed time in hours to days, for comparing fast pathways
/// (such as image-based teledermatology triage) against day-scale medians.
///
/// # Arguments
///
/// * `hours` — elapsed time in hours.
///
/// # Returns
///
/// The equivalent time in days: `hours / 24`.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_referral_turnaround_time::hours_to_days;
///
/// // The doc's teledermatology worked example: a 4-hour median.
/// let days = hours_to_days(4.0);
/// assert!((days - (4.0 / 24.0)).abs() < 1e-9);
/// ```
#[must_use]
pub fn hours_to_days(hours: f64) -> f64 {
    hours / 24.0
}

/// Empirical percentile (0–100) of a set of turnaround-time durations, by
/// linear interpolation between order statistics.
///
/// The doc's worked example reports the median (50th percentile) and 90th
/// percentile rather than the mean, because the distribution is heavily
/// right-skewed by returned or complex referrals.
///
/// # Arguments
///
/// * `durations` — turnaround times, in any consistent unit (unsorted is
///   fine; a sorted copy is made).
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
/// use digital_health::digital_referral_turnaround_time::percentile;
///
/// let durations = [0.3, 0.5, 0.8, 1.2, 1.8, 1.8, 2.5, 3.5, 6.0, 6.0];
/// assert_eq!(percentile(&durations, 50.0), Some(1.8));
/// assert_eq!(percentile(&durations, 90.0), Some(6.0));
/// assert!(percentile(&[], 50.0).is_none());
/// ```
///
#[must_use]
pub fn percentile(durations: &[f64], p: f64) -> Option<f64> {
    crate::internal::percentile(durations, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "a median time from submission to triage decision of 1.8
    // days across all specialties, with a 90th-percentile time of 6 days".
    #[test]
    fn worked_example_median_and_p90_match_doc() {
        let durations = [0.3, 0.5, 0.8, 1.2, 1.8, 1.8, 2.5, 3.5, 6.0, 6.0];
        let median = percentile(&durations, 50.0).unwrap();
        let p90 = percentile(&durations, 90.0).unwrap();
        assert!((median - 1.8).abs() < 1e-9, "got median {median}");
        assert!((p90 - 6.0).abs() < 1e-9, "got p90 {p90}");
    }

    // Doc line: "a teledermatology pathway ... achieves a median turnaround
    // of 4 hours" — well under the 1.8-day general median.
    #[test]
    fn teledermatology_median_is_far_below_general_median() {
        let teledermatology_days = hours_to_days(4.0);
        assert!(teledermatology_days < 1.8, "got {teledermatology_days}");
    }

    #[test]
    fn empty_durations_returns_none() {
        assert!(percentile(&[], 50.0).is_none());
    }
}

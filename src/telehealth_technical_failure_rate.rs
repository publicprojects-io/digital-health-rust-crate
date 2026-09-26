//! # Telehealth Technical Failure Rate
//!
//! Telehealth technical failure rate is the share of attempted telehealth
//! visits that fail to connect or terminate abnormally for a technical
//! reason, and the share of those failures that are successfully rebooked.
//! [Telehealth visit rate](crate::telehealth_visit_rate) already warns
//! against "counting attempted rather than completed visits"; this module
//! is that warning turned into its own measured metric, rather than a
//! caveat that stays unmeasured.
//!
//! ## How it's calculated
//!
//! ```text
//! Technical failure rate = failed attempts / total attempts × 100
//!
//! Rebooking rate = attempts rebooked after failure / failed attempts × 100
//! ```
//!
//! ## Why it matters
//!
//! A platform that silently retries a failed connection and logs only the
//! eventual successful attempt makes technical failure invisible to
//! [telehealth visit rate](crate::telehealth_visit_rate) — the visit still
//! shows up as a completed encounter, and the friction the patient
//! experienced getting there disappears from every report. That friction
//! correlates with the same digital-exclusion factors — older age, limited
//! bandwidth, an unfamiliar device — flagged in [patient portal adoption
//! rate](crate::patient_portal_adoption_rate), so a technical failure rate
//! that goes unmeasured can hide a widening access gap behind a
//! healthy-looking telehealth visit rate. The rebooking rate matters
//! separately: a failure that is never rebooked is functionally a lost
//! visit, similar in effect to [appointment no-show
//! rate](crate::appointment_no_show_rate) but caused by technology rather
//! than patient behaviour, and needing a different fix (device or bandwidth
//! support, not a reminder).
//!
//! ## Worked example
//!
//! A clinic attempts 500 telehealth visits in a month. 35 fail to connect
//! and are logged as technical failures; of those 35, 28 are successfully
//! rebooked within the same week.
//!
//! ```rust
//! use digital_health::telehealth_technical_failure_rate::{technical_failure_rate, rebooking_rate};
//!
//! // 35 / 500 × 100 = 7%.
//! let failure_rate = technical_failure_rate(35.0, 500.0).unwrap();
//! assert!((failure_rate - 7.0).abs() < 1e-9);
//!
//! // 28 / 35 × 100 = 80% of failures are recovered; the remaining 20% are
//! // visits that, absent further follow-up, simply didn't happen.
//! let rebooking = rebooking_rate(28.0, 35.0).unwrap();
//! assert!((rebooking - 80.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The telehealth platform's own session log is the primary source — it
//! must record a failed or abnormally terminated session as a distinct
//! event, not merely omit it, or this metric cannot be calculated at all.
//! Rebooking must then be traced back to the specific failed attempt it
//! followed, typically via the practice's scheduling system.
//!
//! ## Pitfalls
//!
//! - **A platform that only logs the final successful attempt** — if failed
//!   connections aren't retained as events, this metric is unmeasurable
//!   regardless of how the formula is defined; that's a platform
//!   configuration or vendor question to raise before trying to report it.
//! - **Not distinguishing patient-side, clinician-side, and platform-side
//!   failures** — the fix is completely different for each (patient device
//!   or bandwidth support, clinician training, or a vendor issue), and a
//!   blended rate can't tell them apart.
//! - **Conflating a technical failure with a no-show** — the two produce
//!   the same empty slot but have different causes; see [appointment
//!   no-show rate](crate::appointment_no_show_rate)'s caution against
//!   treating non-attendance purely as a behavioural problem.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on telehealth technical barriers and
//!   connectivity failures, published in venues including JMIR and
//!   Telemedicine and e-Health.
//! - ONC / HealthIT.gov and CMS telehealth guidance noting technology
//!   access as a persistent barrier to virtual care.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The technical failure rate: attempted telehealth visits that fail to
/// connect or terminate abnormally, as a percentage of total attempts.
///
/// # Arguments
///
/// * `failed_attempts` — attempted visits that failed to connect or
///   terminated abnormally for a technical reason (count).
/// * `total_attempts` — total attempted telehealth visits in the period
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_attempts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::telehealth_technical_failure_rate::technical_failure_rate;
///
/// // Worked example: 35 / 500 × 100 = 7%.
/// let rate = technical_failure_rate(35.0, 500.0).unwrap();
/// assert!((rate - 7.0).abs() < 1e-9);
///
/// assert!(technical_failure_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn technical_failure_rate(failed_attempts: f64, total_attempts: f64) -> Option<f64> {
    if total_attempts == 0.0 {
        None
    } else {
        Some(failed_attempts / total_attempts * 100.0)
    }
}

/// The rebooking rate: failed attempts that are successfully rebooked, as a
/// percentage of all failed attempts.
///
/// # Arguments
///
/// * `rebooked_after_failure` — failed attempts that were successfully
///   rebooked (count).
/// * `failed_attempts` — total technical failures in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `failed_attempts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::telehealth_technical_failure_rate::rebooking_rate;
///
/// // Worked example: 28 / 35 × 100 = 80%.
/// let rate = rebooking_rate(28.0, 35.0).unwrap();
/// assert!((rate - 80.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn rebooking_rate(rebooked_after_failure: f64, failed_attempts: f64) -> Option<f64> {
    if failed_attempts == 0.0 {
        None
    } else {
        Some(rebooked_after_failure / failed_attempts * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "35 fail to connect ... 7%."
    #[test]
    fn worked_example_technical_failure_rate_is_7_percent() {
        let rate = technical_failure_rate(35.0, 500.0).unwrap();
        assert!((rate - 7.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "28 are successfully rebooked ... 80%."
    #[test]
    fn worked_example_rebooking_rate_is_80_percent() {
        let rate = rebooking_rate(28.0, 35.0).unwrap();
        assert!((rate - 80.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(technical_failure_rate(1.0, 0.0).is_none());
        assert!(rebooking_rate(1.0, 0.0).is_none());
    }
}

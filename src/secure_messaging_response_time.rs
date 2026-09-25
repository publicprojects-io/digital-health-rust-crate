//! # Secure Messaging Response Time
//!
//! Secure messaging response time is the elapsed time from a patient
//! sending a message through a patient portal's secure-messaging ("patient
//! in-basket") feature to a clinician's substantive reply. It is a process
//! metric with two audiences at once: patients experience it directly as
//! responsiveness, and it is one of the most heavily studied contributors
//! to clinician in-basket burden and burnout in the post-portal-adoption
//! literature.
//!
//! ## How it's calculated
//!
//! ```text
//! Response time = timestamp(clinician reply) − timestamp(patient message sent)
//!
//! Report median and a high percentile (commonly the 90th), not only the
//! mean, because a small number of very slow replies — missed or
//! misrouted messages — matter operationally even when the median looks
//! healthy.
//! ```
//!
//! ## Why it matters
//!
//! Secure messaging volume rose sharply with portal adoption, and response
//! time is now tracked both as a patient-experience measure and as a proxy
//! for in-basket workload driving clinician time spent outside scheduled
//! hours (sometimes called "pajama time"). For clinically time-sensitive
//! messages, slow response also carries a direct safety dimension if a
//! practice's triage process doesn't separate urgent messages from routine
//! ones before they queue.
//!
//! ## Worked example
//!
//! A primary care practice logs the elapsed time, in hours, from message
//! sent to clinician reply for ten portal messages in a week: 0.5, 1.0,
//! 1.5, 2.0, 3.0, 3.0, 5.0, 8.0, 20.0, and 20.0 hours.
//!
//! ```rust
//! use digital_health::secure_messaging_response_time::{response_hours, percentile};
//!
//! // A single message: sent at hour 9.0 on the practice's timeline, replied
//! // to at hour 12.5 — a 3.5-hour response.
//! let elapsed = response_hours(9.0, 12.5);
//! assert!((elapsed - 3.5).abs() < 1e-9);
//!
//! // A week of response times (hours), right-skewed by a couple of messages
//! // that sat for most of a working day.
//! let times = [0.5, 1.0, 1.5, 2.0, 3.0, 3.0, 5.0, 8.0, 20.0, 20.0];
//! let median = percentile(&times, 50.0).unwrap();
//! let p90 = percentile(&times, 90.0).unwrap();
//! assert!((median - 3.0).abs() < 1e-9);
//! assert!((p90 - 20.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The electronic health record or portal platform's in-basket audit log is
//! the primary source (message-sent and first-reply timestamps). Where the
//! platform logs both an automated acknowledgement and a later substantive
//! clinician reply, use the substantive reply's timestamp — an
//! acknowledgement is not a response.
//!
//! ## Pitfalls
//!
//! - **Counting an auto-acknowledgement as the reply** — masks the true
//!   clinician response time behind an instant, automated "received"
//!   message.
//! - **Reporting only the mean** — hides the long tail that matters most for
//!   missed-message risk, the same lesson as [digital referral turnaround
//!   time](crate::digital_referral_turnaround_time).
//! - **Not separating message type or urgency** — a routine refill request
//!   and an urgent symptom report have very different appropriate response
//!   targets; blending them obscures whether urgent messages are actually
//!   triaged faster.
//! - **Business-hours vs. calendar-hours clock** — a message sent Friday
//!   evening and answered Monday morning shows a large elapsed-hours figure
//!   that may reflect an intended off-hours policy rather than a failure;
//!   state which clock convention is in use alongside the figure.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on patient portal secure-message burden and
//!   its association with clinician burnout (e.g. studies published in
//!   JAMIA and JAMA Internal Medicine on EHR in-basket time).
//! - ONC / HealthIT.gov guidance on patient portal secure messaging as a
//!   patient-engagement and interoperability measure.
//! - Practice-management and EHR vendor guidance on in-basket triage and
//!   response-time service targets.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written; see Sources above.

/// Elapsed time between a patient's message and a clinician's reply.
///
/// # Arguments
///
/// * `sent_hour` — the message's sent timestamp, expressed as an hour
///   number on any consistent clock (e.g. hours since an epoch).
/// * `replied_hour` — the reply's timestamp, on the same clock.
///
/// # Returns
///
/// Elapsed time in hours: `replied_hour − sent_hour`.
///
/// # Examples
///
/// ```rust
/// use digital_health::secure_messaging_response_time::response_hours;
///
/// let elapsed = response_hours(9.0, 12.5);
/// assert!((elapsed - 3.5).abs() < 1e-9);
/// ```
#[must_use]
pub fn response_hours(sent_hour: f64, replied_hour: f64) -> f64 {
    replied_hour - sent_hour
}

/// Empirical percentile (0–100) of a set of response-time durations, by
/// linear interpolation between order statistics.
///
/// The doc's worked example reports the median (50th percentile) and 90th
/// percentile rather than the mean, because a small number of very slow
/// replies matter operationally even when the median looks healthy.
///
/// # Arguments
///
/// * `response_times` — response times, in any consistent unit (unsorted is
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
/// use digital_health::secure_messaging_response_time::percentile;
///
/// let times = [0.5, 1.0, 1.5, 2.0, 3.0, 3.0, 5.0, 8.0, 20.0, 20.0];
/// assert_eq!(percentile(&times, 50.0), Some(3.0));
/// assert_eq!(percentile(&times, 90.0), Some(20.0));
/// assert!(percentile(&[], 50.0).is_none());
/// ```
#[must_use]
pub fn percentile(response_times: &[f64], p: f64) -> Option<f64> {
    crate::internal::percentile(response_times, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "sent at hour 9.0 ... replied to at hour 12.5 — a 3.5-hour
    // response."
    #[test]
    fn worked_example_response_hours_is_3_5() {
        let elapsed = response_hours(9.0, 12.5);
        assert!((elapsed - 3.5).abs() < 1e-9, "got {elapsed}");
    }

    // Doc line: "The median response time is 3.0 hours; the 90th
    // percentile ... is 20.0 hours."
    #[test]
    fn worked_example_median_and_p90_match_doc() {
        let times = [0.5, 1.0, 1.5, 2.0, 3.0, 3.0, 5.0, 8.0, 20.0, 20.0];
        let median = percentile(&times, 50.0).unwrap();
        let p90 = percentile(&times, 90.0).unwrap();
        assert!((median - 3.0).abs() < 1e-9, "got median {median}");
        assert!((p90 - 20.0).abs() < 1e-9, "got p90 {p90}");
    }

    #[test]
    fn empty_response_times_returns_none() {
        assert!(percentile(&[], 50.0).is_none());
    }
}

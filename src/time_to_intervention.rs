//! # Time to Intervention
//!
//! Time to intervention is the elapsed time between an automated health
//! alert — a remote-monitoring threshold breach, a deterioration score —
//! and a clinical team's first response, and the share of alerts answered
//! within a target time. It is the process metric that shows whether
//! automated alerting actually speeds up care compared with a traditional
//! in-clinic phone call.
//!
//! ## How it's calculated
//!
//! ```text
//! Time to intervention (minutes) = intervention minute − alert minute
//!
//! Within-target rate = alerts answered within target / alerts × 100
//! ```
//!
//! ## Why it matters
//!
//! An alert nobody acts on quickly has no clinical value, and a median
//! hides the slow tail where harm happens. Reporting a percentile and a
//! within-target share together shows both the typical and the worst-case
//! response. It is the alert-side counterpart to
//! [`secure_messaging_response_time`](crate::secure_messaging_response_time)
//! and pairs with [`clinical_alert_firing_rate`](crate::clinical_alert_firing_rate)
//! for the alert-fatigue picture.
//!
//! ## Worked example
//!
//! A remote-monitoring service raises an alert at minute 100 of a shift and
//! a nurse responds at minute 118. Over a month, 720 of 800 alerts are
//! answered within the 30-minute target. Five response times, in minutes,
//! are 5, 10, 15, 20 and 25.
//!
//! ```rust
//! use digital_health::time_to_intervention::{elapsed_minutes, percentile, within_target_rate};
//!
//! // 118 − 100 = 18 minutes.
//! assert!((elapsed_minutes(100.0, 118.0) - 18.0).abs() < 1e-9);
//!
//! // 720 / 800 × 100 = 90% within target.
//! let rate = within_target_rate(720.0, 800.0).unwrap();
//! assert!((rate - 90.0).abs() < 1e-9);
//!
//! // The median of [5, 10, 15, 20, 25] is 15 minutes.
//! let times = [5.0, 10.0, 15.0, 20.0, 25.0];
//! assert!((percentile(&times, 50.0).unwrap() - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Alert time comes from the monitoring platform's event log; intervention
//! time must be a logged clinical action (a call, an order, a message), not
//! an alert being opened or acknowledged. Decide once how out-of-hours
//! alerts are counted.
//!
//! ## Pitfalls
//!
//! - **Counting acknowledgement as intervention** — opening an alert is not
//!   a clinical response.
//! - **Reporting only the mean** — a long tail of slow responses is the
//!   safety risk; use [`percentile`].
//! - **Comparing with phone-based pathways on different clocks** — measure
//!   both from the moment the abnormal value occurred.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on remote patient monitoring alert response
//!   and time to clinical action.
//! - NHS England and CMS guidance on remote monitoring service standards.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The time to intervention, in minutes: the intervention timestamp minus
/// the alert timestamp.
///
/// # Arguments
///
/// * `alert_minute` — when the alert was raised, in minutes on any
///   consistent clock.
/// * `intervention_minute` — when a clinician first acted, on the same
///   clock.
///
/// # Returns
///
/// The elapsed minutes. Always defined — there is no denominator.
///
/// # Examples
///
/// ```rust
/// use digital_health::time_to_intervention::elapsed_minutes;
///
/// // Worked example: 118 − 100 = 18 minutes.
/// assert!((elapsed_minutes(100.0, 118.0) - 18.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn elapsed_minutes(alert_minute: f64, intervention_minute: f64) -> f64 {
    intervention_minute - alert_minute
}

/// The within-target rate: alerts answered within the target time as a
/// percentage of alerts raised.
///
/// # Arguments
///
/// * `within_target` — alerts with an intervention inside the target time
///   (count).
/// * `alerts` — alerts raised in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `alerts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::time_to_intervention::within_target_rate;
///
/// // Worked example: 720 / 800 × 100 = 90%.
/// let rate = within_target_rate(720.0, 800.0).unwrap();
/// assert!((rate - 90.0).abs() < 1e-9);
///
/// assert!(within_target_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn within_target_rate(within_target: f64, alerts: f64) -> Option<f64> {
    if alerts == 0.0 {
        None
    } else {
        Some(within_target / alerts * 100.0)
    }
}

/// The empirical percentile of a set of response times, by linear
/// interpolation between order statistics.
///
/// # Arguments
///
/// * `response_times` — individual times to intervention, in any
///   consistent unit.
/// * `p` — the percentile, from 0 to 100; values outside are clamped.
///
/// # Returns
///
/// `Some(value)`, or `None` if `response_times` is empty.
///
/// # Examples
///
/// ```rust
/// use digital_health::time_to_intervention::percentile;
///
/// // Worked example: the median of [5, 10, 15, 20, 25] is 15.
/// let times = [5.0, 10.0, 15.0, 20.0, 25.0];
/// assert!((percentile(&times, 50.0).unwrap() - 15.0).abs() < 1e-9);
///
/// assert!(percentile(&[], 50.0).is_none());
/// ```
#[must_use]
pub fn percentile(response_times: &[f64], p: f64) -> Option<f64> {
    crate::internal::percentile(response_times, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "alert at minute 100 ... responds at minute 118" — 18 minutes.
    #[test]
    fn worked_example_elapsed_minutes_is_18() {
        assert!((elapsed_minutes(100.0, 118.0) - 18.0).abs() < 1e-9);
    }

    // Doc line: "720 of 800 alerts are answered within the target" — 90%.
    #[test]
    fn worked_example_within_target_rate_is_90_percent() {
        let rate = within_target_rate(720.0, 800.0).unwrap();
        assert!((rate - 90.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "5, 10, 15, 20 and 25" — median 15.
    #[test]
    fn worked_example_median_is_15() {
        let times = [5.0, 10.0, 15.0, 20.0, 25.0];
        assert!((percentile(&times, 50.0).unwrap() - 15.0).abs() < 1e-9);
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(within_target_rate(1.0, 0.0).is_none());
    }

    #[test]
    fn empty_times_returns_none() {
        assert!(percentile(&[], 50.0).is_none());
    }
}

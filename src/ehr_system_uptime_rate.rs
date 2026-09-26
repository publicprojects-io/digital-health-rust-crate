//! # EHR System Uptime Rate
//!
//! EHR system uptime rate is the share of a measurement period during which
//! a clinical system is available, and the share of total downtime that was
//! unplanned rather than scheduled maintenance. Essentially every other
//! metric in this crate implicitly assumes the underlying system is
//! available; an unmeasured downtime period can silently distort all of
//! them at once.
//!
//! ## How it's calculated
//!
//! ```text
//! Uptime rate = uptime minutes / total minutes in period × 100
//!
//! Unplanned downtime share = unplanned downtime minutes / total downtime minutes × 100
//! ```
//!
//! ## Why it matters
//!
//! A portal adoption rate measured during an outage understates true
//! engagement; a CPOE electronic-order rate that drops during downtime
//! looks like an adoption regression rather than an infrastructure event.
//! Distinguishing planned from unplanned downtime matters because the two
//! have completely different remediation paths — a maintenance-window
//! scheduling change versus an infrastructure reliability investigation —
//! so a blended downtime figure obscures which one is needed.
//!
//! ## Worked example
//!
//! Over a measurement period of 40,000 minutes, an EHR system logs 200
//! minutes of total downtime, leaving 39,800 minutes during which the
//! system is available; of the 200 downtime minutes, 150 are unplanned.
//!
//! ```rust
//! use digital_health::ehr_system_uptime_rate::{uptime_rate, unplanned_downtime_share};
//!
//! // 39,800 / 40,000 × 100 = 99.5%.
//! let uptime = uptime_rate(39_800.0, 40_000.0).unwrap();
//! assert!((uptime - 99.5).abs() < 1e-9);
//!
//! // 150 / 200 × 100 = 75% of downtime was unplanned.
//! let unplanned = unplanned_downtime_share(150.0, 200.0).unwrap();
//! assert!((unplanned - 75.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The system's own availability monitoring log, or the IT service
//! management (ITSM) platform's incident and maintenance-window records,
//! are the primary sources.
//!
//! ## Pitfalls
//!
//! - **Reporting a single blended uptime figure** — without separating
//!   planned from unplanned downtime, the figure can't distinguish a
//!   maintenance-scheduling issue from a reliability problem.
//! - **Measuring the wrong scope** — the core EHR can be up while a
//!   dependent integration (e.g. an e-prescribing gateway) is down and
//!   still causing real clinical impact a narrow core-system uptime measure
//!   misses.
//! - **Not correlating downtime with other metrics** — an anomaly in any
//!   other metric in this crate during a reporting period that included
//!   significant downtime should be checked against the downtime log before
//!   being read as a genuine behavioural change.
//!
//! ## Sources
//!
//! - HIMSS and ONC health IT infrastructure reliability guidance.
//! - Peer-reviewed literature on EHR downtime and its clinical and
//!   operational impact.
//! - IT service management (ITIL) uptime and availability measurement
//!   conventions, as applied to clinical systems.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The uptime rate: minutes the system was available, as a percentage of
/// total minutes in the measurement period.
///
/// # Arguments
///
/// * `uptime_minutes` — minutes the system was available (count).
/// * `total_minutes` — total minutes in the measurement period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_minutes` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::ehr_system_uptime_rate::uptime_rate;
///
/// // Worked example: 39,800 / 40,000 × 100 = 99.5%.
/// let rate = uptime_rate(39_800.0, 40_000.0).unwrap();
/// assert!((rate - 99.5).abs() < 1e-9);
///
/// assert!(uptime_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn uptime_rate(uptime_minutes: f64, total_minutes: f64) -> Option<f64> {
    if total_minutes == 0.0 {
        None
    } else {
        Some(uptime_minutes / total_minutes * 100.0)
    }
}

/// The unplanned downtime share: unplanned downtime minutes, as a
/// percentage of total downtime minutes.
///
/// # Arguments
///
/// * `unplanned_downtime_minutes` — downtime minutes that were unplanned,
///   i.e. not scheduled maintenance (count).
/// * `total_downtime_minutes` — total downtime minutes in the period
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_downtime_minutes` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::ehr_system_uptime_rate::unplanned_downtime_share;
///
/// // Worked example: 150 / 200 × 100 = 75%.
/// let rate = unplanned_downtime_share(150.0, 200.0).unwrap();
/// assert!((rate - 75.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn unplanned_downtime_share(
    unplanned_downtime_minutes: f64,
    total_downtime_minutes: f64,
) -> Option<f64> {
    if total_downtime_minutes == 0.0 {
        None
    } else {
        Some(unplanned_downtime_minutes / total_downtime_minutes * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "39,800 the system is available" out of 40,000 — 99.5%.
    #[test]
    fn worked_example_uptime_rate_is_99_5_percent() {
        let rate = uptime_rate(39_800.0, 40_000.0).unwrap();
        assert!((rate - 99.5).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "150 are unplanned" out of 200 downtime minutes — 75%.
    #[test]
    fn worked_example_unplanned_downtime_share_is_75_percent() {
        let rate = unplanned_downtime_share(150.0, 200.0).unwrap();
        assert!((rate - 75.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(uptime_rate(1.0, 0.0).is_none());
        assert!(unplanned_downtime_share(1.0, 0.0).is_none());
    }
}

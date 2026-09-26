//! # Clinical Alert Firing Rate
//!
//! Clinical alert firing rate is the share of clinical orders that trigger
//! at least one clinical decision support (CDS) alert, and the share of
//! fired alerts that are high-severity. Where [clinical alert override
//! rate](crate::clinical_alert_override_rate) measures what happens *after*
//! an alert fires, this module measures how often alerts fire in the first
//! place — the other half of the alert-fatigue picture.
//!
//! ## How it's calculated
//!
//! ```text
//! Alert firing rate = alerts fired / orders placed × 100
//!
//! High-severity firing share = high-severity alerts fired / total alerts fired × 100
//! ```
//!
//! ## Why it matters
//!
//! A system tuned to fire fewer, higher-value alerts shows both a lower
//! firing rate and a higher high-severity share among the alerts that do
//! fire — the direction alert-fatigue remediation efforts aim for. Firing
//! rate alone, without the severity mix, can't distinguish "a well-tuned
//! system firing rarely" from "alert categories got disabled wholesale,"
//! which is exactly the ambiguity [clinical alert override
//! rate](crate::clinical_alert_override_rate)'s own rustdoc warns against
//! for the override side of the same picture.
//!
//! ## Worked example
//!
//! A hospital's CPOE system processes 10,000 medication orders in a month;
//! 4,000 of them trigger at least one alert. Of those 4,000 fired alerts,
//! 500 are high-severity (contraindicated or major).
//!
//! ```rust
//! use digital_health::clinical_alert_firing_rate::{alert_firing_rate, high_severity_firing_share};
//!
//! // 4,000 / 10,000 × 100 = 40%.
//! let firing = alert_firing_rate(4_000.0, 10_000.0).unwrap();
//! assert!((firing - 40.0).abs() < 1e-9);
//!
//! // 500 / 4,000 × 100 = 12.5%.
//! let high_severity = high_severity_firing_share(500.0, 4_000.0).unwrap();
//! assert!((high_severity - 12.5).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The CDS engine's or EHR's own alert-firing audit log is the primary
//! source, using its per-order alert count and each alert's severity tier.
//!
//! ## Pitfalls
//!
//! - **Comparing firing rates across systems with different alert rule
//!   sets** — a system with fewer enabled rules will show a lower firing
//!   rate independent of true alert quality.
//! - **Tracking firing rate without the severity mix** — see [clinical alert
//!   override rate](crate::clinical_alert_override_rate)'s caution against
//!   treating a single rate as a safety score; the same applies here in
//!   reverse.
//! - **Conflating genuine alert-rule tuning with wholesale rule
//!   disabling** — both lower the firing rate, but only one is a safety
//!   improvement.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on clinical decision support alert fatigue,
//!   the same evidence base as [clinical alert override
//!   rate](crate::clinical_alert_override_rate), published in venues
//!   including JAMIA and npj Digital Medicine.
//! - ONC / HealthIT.gov health IT safety guidance on clinical decision
//!   support.
//! - Institute for Safe Medication Practices (ISMP), guidance on CDS alert
//!   design.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The alert firing rate: orders that trigger at least one alert, as a
/// percentage of total orders placed.
///
/// # Arguments
///
/// * `alerts_fired` — orders that triggered at least one CDS alert (count).
/// * `orders_placed` — total orders placed in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `orders_placed` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::clinical_alert_firing_rate::alert_firing_rate;
///
/// // Worked example: 4,000 / 10,000 × 100 = 40%.
/// let rate = alert_firing_rate(4_000.0, 10_000.0).unwrap();
/// assert!((rate - 40.0).abs() < 1e-9);
///
/// assert!(alert_firing_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn alert_firing_rate(alerts_fired: f64, orders_placed: f64) -> Option<f64> {
    if orders_placed == 0.0 {
        None
    } else {
        Some(alerts_fired / orders_placed * 100.0)
    }
}

/// The high-severity firing share: high-severity alerts, as a percentage of
/// all fired alerts.
///
/// # Arguments
///
/// * `high_severity_alerts` — fired alerts classified as high severity, e.g.
///   contraindicated or major (count).
/// * `alerts_fired` — total alerts fired in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `alerts_fired` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::clinical_alert_firing_rate::high_severity_firing_share;
///
/// // Worked example: 500 / 4,000 × 100 = 12.5%.
/// let rate = high_severity_firing_share(500.0, 4_000.0).unwrap();
/// assert!((rate - 12.5).abs() < 1e-9);
/// ```
#[must_use]
pub fn high_severity_firing_share(high_severity_alerts: f64, alerts_fired: f64) -> Option<f64> {
    if alerts_fired == 0.0 {
        None
    } else {
        Some(high_severity_alerts / alerts_fired * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "4,000 of them trigger at least one alert" — 40%.
    #[test]
    fn worked_example_alert_firing_rate_is_40_percent() {
        let rate = alert_firing_rate(4_000.0, 10_000.0).unwrap();
        assert!((rate - 40.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "500 are high-severity" out of 4,000 fired — 12.5%.
    #[test]
    fn worked_example_high_severity_share_is_12_5_percent() {
        let rate = high_severity_firing_share(500.0, 4_000.0).unwrap();
        assert!((rate - 12.5).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(alert_firing_rate(1.0, 0.0).is_none());
        assert!(high_severity_firing_share(1.0, 0.0).is_none());
    }
}

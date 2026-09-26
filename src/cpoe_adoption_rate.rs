//! # CPOE Adoption Rate
//!
//! Computerized provider order entry (CPOE) adoption rate is the share of
//! clinical orders — medications, labs, imaging — placed directly into the
//! electronic health record by the ordering clinician, rather than given
//! verbally or written on paper. It is one of the oldest and most
//! foundational digital-health measures, a core meaningful-use and
//! Promoting Interoperability Program metric, and the precondition for
//! [clinical alert override rate](crate::clinical_alert_override_rate) to
//! mean anything at all: an order that never enters CPOE never has the
//! chance to trigger — or be overridden by — a dose-range, interaction, or
//! allergy check.
//!
//! ## How it's calculated
//!
//! ```text
//! Electronic order rate = orders placed via CPOE / total orders × 100
//!
//! Verbal order rate = verbal orders / total orders × 100
//! ```
//!
//! Call each once per order type (medication, lab, imaging) where the
//! underlying volumes are segmented that way.
//!
//! ## Why it matters
//!
//! A verbal order bypasses every CPOE safety check entirely — it never
//! generates an alert for a clinician to override, so it is invisible to
//! [clinical alert override rate](crate::clinical_alert_override_rate) even
//! though it can carry the same dosing or interaction risk a CPOE order
//! would have caught. A rising verbal-order rate can therefore mask a
//! growing pocket of unmitigated medication risk behind an apparently
//! healthy, low override rate. CPOE adoption itself is also the
//! precondition for essentially every other digital medication-safety
//! measure in this crate: none of them apply to an order that was never
//! entered electronically.
//!
//! ## Worked example
//!
//! A hospital places 10,000 medication orders in a month: 9,400 via CPOE,
//! 500 given verbally (and later countersigned), and the remainder on paper
//! during a brief system outage.
//!
//! ```rust
//! use digital_health::cpoe_adoption_rate::{electronic_order_rate, verbal_order_rate};
//!
//! // 9,400 / 10,000 × 100 = 94%.
//! let electronic = electronic_order_rate(9_400.0, 10_000.0).unwrap();
//! assert!((electronic - 94.0).abs() < 1e-9);
//!
//! // 500 / 10,000 × 100 = 5% — invisible to the override rate entirely,
//! // since a verbal order never generates an alert to override.
//! let verbal = verbal_order_rate(500.0, 10_000.0).unwrap();
//! assert!((verbal - 5.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The electronic health record's order-entry audit log is the primary
//! source, using its order-channel field (electronic, verbal, written); the
//! metric's quality depends on clinicians and transcribing staff
//! consistently recording verbal orders as verbal rather than
//! retrospectively entering them as if they had been placed electronically.
//!
//! ## Pitfalls
//!
//! - **Treating high CPOE adoption as sufficient for medication safety on
//!   its own** — an order placed through CPOE can still trigger an alert
//!   that gets overridden unsafely; see [clinical alert override
//!   rate](crate::clinical_alert_override_rate) for the metric that measures
//!   what happens after the order is placed.
//! - **Not tracking verbal orders separately from paper orders** — the two
//!   have different causes and different remediations; verbal orders often
//!   persist even in fully CPOE-enabled units for legitimate reasons (e.g.
//!   during a code), and need a documented read-back-and-verify policy
//!   rather than an elimination target.
//! - **Reporting a rate that spans a system outage without saying so** — a
//!   paper-order spike during EHR downtime reflects infrastructure
//!   resilience, not clinician behaviour, and will otherwise be
//!   misread as a training or adoption gap.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov, Promoting Interoperability Program CPOE measures.
//! - Institute for Safe Medication Practices (ISMP), guidance on verbal
//!   orders and CPOE-related medication safety.
//! - Peer-reviewed literature on CPOE adoption and medication error
//!   reduction.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The electronic order rate: orders placed via CPOE as a percentage of
/// total orders.
///
/// # Arguments
///
/// * `electronic_orders` — orders placed directly into the EHR by the
///   ordering clinician (count).
/// * `total_orders` — total orders in the period, or order type (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_orders` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::cpoe_adoption_rate::electronic_order_rate;
///
/// // Worked example: 9,400 / 10,000 × 100 = 94%.
/// let rate = electronic_order_rate(9_400.0, 10_000.0).unwrap();
/// assert!((rate - 94.0).abs() < 1e-9);
///
/// assert!(electronic_order_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn electronic_order_rate(electronic_orders: f64, total_orders: f64) -> Option<f64> {
    if total_orders == 0.0 {
        None
    } else {
        Some(electronic_orders / total_orders * 100.0)
    }
}

/// The verbal order rate: orders given verbally as a percentage of total
/// orders.
///
/// Verbal orders bypass CPOE's safety checks entirely, so this rate is the
/// signal for a class of medication risk that [clinical alert override
/// rate](crate::clinical_alert_override_rate) cannot see.
///
/// # Arguments
///
/// * `verbal_orders` — orders given verbally, typically countersigned later
///   (count).
/// * `total_orders` — total orders in the period, or order type (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_orders` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::cpoe_adoption_rate::verbal_order_rate;
///
/// // Worked example: 500 / 10,000 × 100 = 5%.
/// let rate = verbal_order_rate(500.0, 10_000.0).unwrap();
/// assert!((rate - 5.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn verbal_order_rate(verbal_orders: f64, total_orders: f64) -> Option<f64> {
    if total_orders == 0.0 {
        None
    } else {
        Some(verbal_orders / total_orders * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "9,400 via CPOE ... 94%."
    #[test]
    fn worked_example_electronic_order_rate_is_94_percent() {
        let rate = electronic_order_rate(9_400.0, 10_000.0).unwrap();
        assert!((rate - 94.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "500 given verbally ... 5%."
    #[test]
    fn worked_example_verbal_order_rate_is_5_percent() {
        let rate = verbal_order_rate(500.0, 10_000.0).unwrap();
        assert!((rate - 5.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(electronic_order_rate(1.0, 0.0).is_none());
        assert!(verbal_order_rate(1.0, 0.0).is_none());
    }
}

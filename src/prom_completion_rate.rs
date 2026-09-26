//! # PROM Completion Rate
//!
//! Patient-reported outcome measure (PROM) completion rate is the share of
//! eligible patients who complete a PROM questionnaire, and the share of
//! completions gathered through a digital channel (app, portal, or SMS
//! link) rather than paper or a phone call. PROM data is only useful for
//! outcome measurement and value-based care contracts if the completing
//! population is representative of everyone treated, which is why
//! completion rate — not just the outcome scores themselves — is tracked as
//! a measure in its own right.
//!
//! ## How it's calculated
//!
//! ```text
//! PROM completion rate = PROMs completed / eligible patients × 100
//!
//! Digital completion share = PROMs completed digitally / PROMs completed × 100
//! ```
//!
//! ## Why it matters
//!
//! A low completion rate, or one skewed heavily toward a single channel,
//! risks non-response bias: patients who are sicker, older, or less
//! digitally engaged are systematically under-represented, which can make
//! an outcome look better than it truly is across the full treated
//! population. This is the same digital-exclusion caution [patient portal
//! adoption rate](crate::patient_portal_adoption_rate) raises about its own
//! adoption figures.
//!
//! ## Worked example
//!
//! An orthopaedic joint-replacement programme identifies 800 eligible
//! patients for a 6-month post-operative PROM. 560 complete it; of those,
//! 420 complete it via the digital (app or SMS-link) channel.
//!
//! ```rust
//! use digital_health::prom_completion_rate::{prom_completion_rate, digital_completion_share};
//!
//! // 560 / 800 × 100 = 70%.
//! let completion = prom_completion_rate(560.0, 800.0).unwrap();
//! assert!((completion - 70.0).abs() < 1e-9);
//!
//! // 420 / 560 × 100 = 75% complete digitally.
//! let digital_share = digital_completion_share(420.0, 560.0).unwrap();
//! assert!((digital_share - 75.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The PROM collection platform's own response log is the primary source,
//! using its completion channel field (digital versus paper or phone) and
//! its follow-up window relative to the index procedure or visit date.
//!
//! ## Pitfalls
//!
//! - **Reporting completion rate without checking representativeness** — a
//!   completion rate can be high overall while skewed heavily away from the
//!   patients most likely to have a worse outcome.
//! - **Treating digital-channel completion as a costless substitute for
//!   paper** — it can simply filter out less digitally-engaged patients
//!   rather than capturing them more efficiently.
//! - **Comparing completion rates across programmes with different
//!   follow-up windows** — a 90-day follow-up and a 12-month follow-up are
//!   not directly comparable.
//!
//! ## Sources
//!
//! - NHS PROMs programme methodology and publications.
//! - CMS quality reporting guidance on patient-reported outcome measures.
//! - Peer-reviewed literature on PROM response bias and digital collection
//!   methods.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The PROM completion rate: completed patient-reported outcome measures as
/// a percentage of eligible patients.
///
/// # Arguments
///
/// * `completed` — eligible patients who completed the PROM (count).
/// * `eligible` — total eligible patients in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `eligible` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::prom_completion_rate::prom_completion_rate;
///
/// // Worked example: 560 / 800 × 100 = 70%.
/// let rate = prom_completion_rate(560.0, 800.0).unwrap();
/// assert!((rate - 70.0).abs() < 1e-9);
///
/// assert!(prom_completion_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn prom_completion_rate(completed: f64, eligible: f64) -> Option<f64> {
    if eligible == 0.0 {
        None
    } else {
        Some(completed / eligible * 100.0)
    }
}

/// The digital completion share: PROMs completed via a digital channel, as
/// a percentage of all completed PROMs.
///
/// # Arguments
///
/// * `completed_digitally` — completed PROMs gathered through a digital
///   channel — app, portal, or SMS link (count).
/// * `completed` — total completed PROMs in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `completed` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::prom_completion_rate::digital_completion_share;
///
/// // Worked example: 420 / 560 × 100 = 75%.
/// let rate = digital_completion_share(420.0, 560.0).unwrap();
/// assert!((rate - 75.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn digital_completion_share(completed_digitally: f64, completed: f64) -> Option<f64> {
    if completed == 0.0 {
        None
    } else {
        Some(completed_digitally / completed * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "560 complete it" out of 800 eligible — 70%.
    #[test]
    fn worked_example_prom_completion_rate_is_70_percent() {
        let rate = prom_completion_rate(560.0, 800.0).unwrap();
        assert!((rate - 70.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "420 complete it via the digital ... channel" — 75%.
    #[test]
    fn worked_example_digital_completion_share_is_75_percent() {
        let rate = digital_completion_share(420.0, 560.0).unwrap();
        assert!((rate - 75.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(prom_completion_rate(1.0, 0.0).is_none());
        assert!(digital_completion_share(1.0, 0.0).is_none());
    }
}

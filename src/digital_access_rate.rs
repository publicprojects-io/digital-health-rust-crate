//! # Digital Access Rate
//!
//! Digital access rate is the share of a population with the connectivity
//! or device access needed to use a digital health service — broadband at
//! home, a smartphone, or an activated patient portal account. The access
//! parity ratio compares a subgroup's access rate to a reference group's,
//! turning a single overall figure into an equity measure.
//!
//! ## How it's calculated
//!
//! ```text
//! Digital access rate = people with access / population × 100
//!
//! Access parity ratio = subgroup access rate / reference group access rate
//! ```
//!
//! ## Why it matters
//!
//! Frameworks such as the HIMSS Digital Health Equity Measurement Framework
//! treat access as the first equity dimension: an overall access rate can
//! look healthy while rural, older, low-income, or limited-English
//! populations fall well behind. A parity ratio below 1.0 quantifies that
//! gap. Note the ratio is a plain ratio, not a percentage.
//!
//! ## Worked example
//!
//! In a service area of 12,000 households, 8,400 have home broadband. The
//! rural subgroup's broadband rate is 60%, against 80% for the suburban
//! reference group.
//!
//! ```rust
//! use digital_health::digital_access_rate::{access_parity_ratio, digital_access_rate};
//!
//! // 8,400 / 12,000 × 100 = 70%.
//! let access = digital_access_rate(8_400.0, 12_000.0).unwrap();
//! assert!((access - 70.0).abs() < 1e-9);
//!
//! // 60 / 80 = 0.75 — rural access is three quarters of the reference.
//! let parity = access_parity_ratio(60.0, 80.0).unwrap();
//! assert!((parity - 0.75).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Population-level connectivity and device ownership come from sources
//! such as the US Census American Community Survey and NTIA Internet Use
//! Survey; portal usage comes from the organisation's own system, filtered
//! by the same demographic and geographic strata.
//!
//! ## Pitfalls
//!
//! - **Reporting only the overall rate** — it hides exactly the subgroup
//!   gaps the metric exists to expose.
//! - **Treating a device as access** — a shared or data-capped phone is not
//!   equivalent to home broadband.
//! - **Using a different reference group each period** — the parity ratio
//!   is only comparable if the reference stays fixed.
//!
//! ## Sources
//!
//! - HIMSS Digital Health Equity Measurement Framework (DHEMF).
//! - US Census Bureau American Community Survey, computer and internet use
//!   tables.
//! - NTIA Internet Use Survey.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The digital access rate: people with the needed connectivity or device
/// access as a percentage of the population.
///
/// # Arguments
///
/// * `with_access` — people or households with access (count).
/// * `population` — total people or households in scope (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `population` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_access_rate::digital_access_rate;
///
/// // Worked example: 8,400 / 12,000 × 100 = 70%.
/// let rate = digital_access_rate(8_400.0, 12_000.0).unwrap();
/// assert!((rate - 70.0).abs() < 1e-9);
///
/// assert!(digital_access_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn digital_access_rate(with_access: f64, population: f64) -> Option<f64> {
    if population == 0.0 {
        None
    } else {
        Some(with_access / population * 100.0)
    }
}

/// The access parity ratio: a subgroup's access rate divided by a reference
/// group's. A value below 1.0 means the subgroup has less access.
///
/// # Arguments
///
/// * `group_rate` — the subgroup's access rate (percentage).
/// * `reference_rate` — the reference group's access rate (percentage).
///
/// # Returns
///
/// `Some(ratio)` — a plain ratio, not a percentage — or `None` if
/// `reference_rate` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_access_rate::access_parity_ratio;
///
/// // Worked example: 60 / 80 = 0.75.
/// let parity = access_parity_ratio(60.0, 80.0).unwrap();
/// assert!((parity - 0.75).abs() < 1e-9);
///
/// assert!(access_parity_ratio(60.0, 0.0).is_none());
/// ```
#[must_use]
pub fn access_parity_ratio(group_rate: f64, reference_rate: f64) -> Option<f64> {
    if reference_rate == 0.0 {
        None
    } else {
        Some(group_rate / reference_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "8,400 have home broadband" of 12,000 — 70%.
    #[test]
    fn worked_example_digital_access_rate_is_70_percent() {
        let rate = digital_access_rate(8_400.0, 12_000.0).unwrap();
        assert!((rate - 70.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "60% ... against 80%" — parity 0.75.
    #[test]
    fn worked_example_access_parity_ratio_is_0_75() {
        let parity = access_parity_ratio(60.0, 80.0).unwrap();
        assert!((parity - 0.75).abs() < 1e-9, "got {parity}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(digital_access_rate(1.0, 0.0).is_none());
        assert!(access_parity_ratio(60.0, 0.0).is_none());
    }
}

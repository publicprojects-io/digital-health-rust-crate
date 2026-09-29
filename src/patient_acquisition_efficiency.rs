//! # Patient Acquisition Efficiency
//!
//! Patient acquisition efficiency measures whether growth spending pays
//! back. The LTV-to-CAC ratio compares a patient's lifetime value to what
//! it cost to acquire them; the marketing efficiency ratio (MER) is total
//! revenue divided by total marketing spend, an independent check on
//! platform-reported return on ad spend.
//!
//! ## How it's calculated
//!
//! ```text
//! LTV to CAC ratio = lifetime value per patient / customer acquisition cost per patient
//!
//! Marketing efficiency ratio = total revenue / total marketing spend
//! ```
//!
//! ## Why it matters
//!
//! A 3:1 LTV-to-CAC ratio is a commonly cited baseline for sustainable
//! growth. MER uses *all* revenue over *all* marketing spend, so unlike
//! ad-platform ROAS it can't be inflated by attribution overlap between
//! channels. Both are dimensionless ratios, not percentages.
//!
//! The functions take plain `f64` amounts in one currency because the
//! result is a ratio. To build a true CAC from its cost components as
//! currency-safe `Money`, see [`patient_acquisition_cost`](crate::patient_acquisition_cost).
//!
//! ## Worked example
//!
//! A digital weight-management programme estimates lifetime value of $1,500
//! per patient against a true acquisition cost of $500. Over the quarter it
//! books $900,000 of revenue on $300,000 of total marketing spend.
//!
//! ```rust
//! use digital_health::patient_acquisition_efficiency::{
//!     ltv_to_cac_ratio, marketing_efficiency_ratio,
//! };
//!
//! // 1,500 / 500 = 3.0, exactly the 3:1 baseline.
//! let ratio = ltv_to_cac_ratio(1_500.0, 500.0).unwrap();
//! assert!((ratio - 3.0).abs() < 1e-9);
//!
//! // 900,000 / 300,000 = 3.0.
//! let mer = marketing_efficiency_ratio(900_000.0, 300_000.0).unwrap();
//! assert!((mer - 3.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Revenue and spend come from finance systems, not ad-platform dashboards.
//! Lifetime value should come from cohort retention and revenue data, using
//! a stated horizon; an assumed multi-year LTV for a service with high
//! early attrition will overstate the ratio.
//!
//! ## Pitfalls
//!
//! - **An undercounted CAC** — omitting agency fees, tooling, or clinical
//!   intake labour flatters the ratio; see
//!   [`patient_acquisition_cost`](crate::patient_acquisition_cost).
//! - **Mixing currencies or time periods** — both arguments must be in the
//!   same currency and cover matching periods.
//! - **Reading MER as profit** — it ignores cost of care delivery and
//!   margin.
//!
//! ## Sources
//!
//! - Standard subscription-business unit-economics literature on LTV:CAC
//!   and its 3:1 benchmark.
//! - Direct-response marketing literature on marketing efficiency ratio as
//!   a blended alternative to platform-reported ROAS.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The LTV-to-CAC ratio: lifetime value per patient divided by customer
/// acquisition cost per patient.
///
/// # Arguments
///
/// * `lifetime_value` — lifetime value per patient.
/// * `acquisition_cost` — customer acquisition cost per patient, in the
///   same currency.
///
/// # Returns
///
/// `Some(ratio)` — a plain ratio, so 3.0 means 3:1 — or `None` if
/// `acquisition_cost` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_acquisition_efficiency::ltv_to_cac_ratio;
///
/// // Worked example: 1,500 / 500 = 3.0.
/// let ratio = ltv_to_cac_ratio(1_500.0, 500.0).unwrap();
/// assert!((ratio - 3.0).abs() < 1e-9);
///
/// assert!(ltv_to_cac_ratio(1_500.0, 0.0).is_none());
/// ```
#[must_use]
pub fn ltv_to_cac_ratio(lifetime_value: f64, acquisition_cost: f64) -> Option<f64> {
    if acquisition_cost == 0.0 {
        None
    } else {
        Some(lifetime_value / acquisition_cost)
    }
}

/// The marketing efficiency ratio: total revenue divided by total marketing
/// spend.
///
/// # Arguments
///
/// * `total_revenue` — all revenue in the period.
/// * `total_marketing_spend` — all marketing spend in the same period and
///   currency.
///
/// # Returns
///
/// `Some(ratio)`, or `None` if `total_marketing_spend` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_acquisition_efficiency::marketing_efficiency_ratio;
///
/// // Worked example: 900,000 / 300,000 = 3.0.
/// let mer = marketing_efficiency_ratio(900_000.0, 300_000.0).unwrap();
/// assert!((mer - 3.0).abs() < 1e-9);
///
/// assert!(marketing_efficiency_ratio(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn marketing_efficiency_ratio(total_revenue: f64, total_marketing_spend: f64) -> Option<f64> {
    if total_marketing_spend == 0.0 {
        None
    } else {
        Some(total_revenue / total_marketing_spend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "$1,500 ... against ... $500" — 3.0.
    #[test]
    fn worked_example_ltv_to_cac_ratio_is_3() {
        let ratio = ltv_to_cac_ratio(1_500.0, 500.0).unwrap();
        assert!((ratio - 3.0).abs() < 1e-9, "got {ratio}");
    }

    // Doc line: "$900,000 of revenue on $300,000" — 3.0.
    #[test]
    fn worked_example_marketing_efficiency_ratio_is_3() {
        let mer = marketing_efficiency_ratio(900_000.0, 300_000.0).unwrap();
        assert!((mer - 3.0).abs() < 1e-9, "got {mer}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(ltv_to_cac_ratio(1.0, 0.0).is_none());
        assert!(marketing_efficiency_ratio(1.0, 0.0).is_none());
    }
}

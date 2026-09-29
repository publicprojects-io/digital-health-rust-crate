//! # Return on Investment
//!
//! Return on investment (ROI) for a digital health platform compares
//! quantifiable benefit — cost savings and revenue — with the total cost of
//! licensing, hardware deployment, and staffing. This module gives the ROI
//! percentage and the benefit-cost ratio from plain amounts in one
//! currency.
//!
//! ## How it's calculated
//!
//! ```text
//! ROI = (total benefit − total cost) / total cost × 100
//!
//! Benefit-cost ratio = total benefit / total cost
//! ```
//!
//! ## Why it matters
//!
//! Value-based care contracts and budget holders ask one question: does it
//! pay back? ROI and benefit-cost ratio are the standard shorthand. Value
//! on investment (VOI) extends ROI with non-financial value such as patient
//! experience, but has no single agreed formula, so is not computed here.
//! To build the cost side as currency-safe `Money`, see
//! [`platform_investment_cost`](crate::platform_investment_cost).
//!
//! ## Worked example
//!
//! A remote-monitoring platform delivers $450,000 of quantified benefit in
//! a year against $300,000 of total cost: licensing, hardware, and
//! staffing.
//!
//! ```rust
//! use digital_health::return_on_investment::{roi_percent, benefit_cost_ratio};
//!
//! // (450,000 − 300,000) / 300,000 × 100 = 50% ROI.
//! let roi_percent_value = roi_percent(450_000.0, 300_000.0).unwrap();
//! assert!((roi_percent_value - 50.0).abs() < 1e-9);
//!
//! // 450,000 / 300,000 = 1.5.
//! let benefit_cost_ratio_value = benefit_cost_ratio(450_000.0, 300_000.0).unwrap();
//! assert!((benefit_cost_ratio_value - 1.5).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Benefit should be attributable and net of what would have happened
//! anyway; cost should include every input, including clinical staff time.
//! Both amounts must cover the same period and currency.
//!
//! ## Pitfalls
//!
//! - Counting gross rather than attributable benefit.
//! - Leaving out staffing or hardware refresh cost.
//! - Comparing ROI over different periods — a 3-year ROI isn't comparable
//!   to a 1-year figure.
//!
//! ## Sources
//!
//! - Standard programme-evaluation and health-economic evaluation
//!   literature on ROI and benefit-cost analysis.
//! - Peer-reviewed literature on ROI of remote patient monitoring
//!   programmes.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The return on investment: net benefit as a percentage of total cost.
///
/// # Arguments
///
/// * `total_benefit` — quantified benefit over the period.
/// * `total_cost` — total programme cost over the same period and currency.
///
/// # Returns
///
/// `Some(percentage)` — negative when cost exceeds benefit — or `None` if
/// `total_cost` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::return_on_investment::roi_percent;
///
/// // Worked example: (450,000 − 300,000) / 300,000 × 100 = 50%.
/// let value = roi_percent(450_000.0, 300_000.0).unwrap();
/// assert!((value - 50.0).abs() < 1e-9);
///
/// assert!(roi_percent(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn roi_percent(total_benefit: f64, total_cost: f64) -> Option<f64> {
    if total_cost == 0.0 {
        None
    } else {
        Some((total_benefit - total_cost) / total_cost * 100.0)
    }
}

/// The benefit-cost ratio: total benefit divided by total cost.
///
/// # Arguments
///
/// * `total_benefit` — quantified benefit over the period.
/// * `total_cost` — total programme cost over the same period and currency.
///
/// # Returns
///
/// `Some(ratio)` — a plain ratio, not a percentage — or `None` if
/// `total_cost` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::return_on_investment::benefit_cost_ratio;
///
/// // Worked example: 450,000 / 300,000 = 1.5.
/// let value = benefit_cost_ratio(450_000.0, 300_000.0).unwrap();
/// assert!((value - 1.5).abs() < 1e-9);
///
/// assert!(benefit_cost_ratio(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn benefit_cost_ratio(total_benefit: f64, total_cost: f64) -> Option<f64> {
    if total_cost == 0.0 {
        None
    } else {
        Some(total_benefit / total_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "(450,000 − 300,000) / 300,000 × 100 = 50% ROI."
    #[test]
    fn worked_example_roi_percent() {
        let value = roi_percent(450_000.0, 300_000.0).unwrap();
        assert!((value - 50.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "450,000 / 300,000 = 1.5."
    #[test]
    fn worked_example_benefit_cost_ratio() {
        let value = benefit_cost_ratio(450_000.0, 300_000.0).unwrap();
        assert!((value - 1.5).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(roi_percent(1.0, 0.0).is_none());
        assert!(benefit_cost_ratio(1.0, 0.0).is_none());
    }
}

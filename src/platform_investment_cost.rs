//! # Platform Investment Cost
//!
//! Platform investment cost is the full cost side of a [return on
//! investment](crate::return_on_investment) calculation: the licensing,
//! hardware deployment, and staffing costs of a digital health platform
//! summed as currency-safe `Money`, and the net benefit once quantified
//! benefit is set against them.
//!
//! ## How it's calculated
//!
//! ```text
//! Total platform cost = licensing cost + hardware cost + staffing cost
//!
//! Net benefit = total benefit − total platform cost
//! ```
//!
//! ## Why it matters
//!
//! ROI is only as honest as its cost figure, and platform cost is routinely
//! undercounted by leaving out clinical staff time or hardware refresh.
//! Summing all three components in one currency, with a typed error on a
//! currency mismatch, gives the figure a payback decision should rest on.
//!
//! ## Worked example
//!
//! A remote-monitoring platform costs an illustrative $120,000.00 in
//! licensing, $80,000.00 in deployed hardware, and $100,000.00 in clinical
//! staffing for the year. Quantified benefit is $450,000.00.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::platform_investment_cost::{total_platform_cost, net_benefit};
//!
//! // $120,000.00 + $80,000.00 + $100,000.00 = $300,000.00.
//! let total_platform_cost_value = total_platform_cost(Money::from_major(120_000, iso::USD), Money::from_major(80_000, iso::USD), Money::from_major(100_000, iso::USD)).unwrap();
//! assert_eq!(total_platform_cost_value, Money::from_major(300_000, iso::USD));
//!
//! // $450,000.00 − $300,000.00 = $150,000.00.
//! let net_benefit_value = net_benefit(Money::from_major(450_000, iso::USD), Money::from_major(300_000, iso::USD)).unwrap();
//! assert_eq!(net_benefit_value, Money::from_major(150_000, iso::USD));
//!
//! ```
//!
//! ## Data sources and caveats
//!
//! Amortise hardware over its useful life and count only the share used in
//! the period; include the clinical staff time spent monitoring,
//! onboarding, and troubleshooting. Benefit must be attributable and cover
//! the same period.
//!
//! ## Pitfalls
//!
//! - Leaving out staffing — usually the largest ongoing cost.
//! - Charging the whole hardware purchase to a single year.
//! - Netting gross rather than attributable benefit.
//!
//! ## Sources
//!
//! - Standard programme-evaluation and health-economic evaluation
//!   literature on cost and benefit measurement.
//! - Peer-reviewed literature on the cost of remote patient monitoring
//!   programmes.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The total platform cost: licensing, hardware, and staffing costs summed.
///
/// # Arguments
///
/// * `licensing_cost` — platform licence or subscription cost.
/// * `hardware_cost` — deployed hardware cost attributable to the period.
/// * `staffing_cost` — clinical and support staffing cost.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if the three costs are not all
/// denominated in the same currency, or [`MoneyError::Overflow`] if the sum
/// overflows.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::platform_investment_cost::total_platform_cost;
///
/// // Worked example: $120,000.00 + $80,000.00 + $100,000.00 = $300,000.00.
/// let value = total_platform_cost(Money::from_major(120_000, iso::USD), Money::from_major(80_000, iso::USD), Money::from_major(100_000, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(300_000, iso::USD));
///
/// assert!(total_platform_cost(Money::from_major(120_000, iso::USD), Money::from_major(80_000, iso::EUR), Money::from_major(100_000, iso::USD)).is_err());
/// ```
pub fn total_platform_cost(
    licensing_cost: Money<'static, iso::Currency>,
    hardware_cost: Money<'static, iso::Currency>,
    staffing_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    licensing_cost.add(hardware_cost)?.add(staffing_cost)
}

/// The net benefit: total quantified benefit minus total platform cost.
///
/// # Arguments
///
/// * `total_benefit` — quantified benefit over the period.
/// * `total_cost` — total platform cost, typically the result of
///   [`total_platform_cost`].
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `total_benefit` and
/// `total_cost` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::platform_investment_cost::net_benefit;
///
/// // Worked example: $450,000.00 − $300,000.00 = $150,000.00.
/// let value = net_benefit(Money::from_major(450_000, iso::USD), Money::from_major(300_000, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(150_000, iso::USD));
///
/// assert!(net_benefit(Money::from_major(450_000, iso::USD), Money::from_major(300_000, iso::EUR)).is_err());
/// ```
pub fn net_benefit(
    total_benefit: Money<'static, iso::Currency>,
    total_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    total_benefit.sub(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "$120,000.00 + $80,000.00 + $100,000.00 = $300,000.00."
    #[test]
    fn worked_example_total_platform_cost() {
        let value = total_platform_cost(
            Money::from_major(120_000, iso::USD),
            Money::from_major(80_000, iso::USD),
            Money::from_major(100_000, iso::USD),
        )
        .unwrap();
        assert_eq!(value, Money::from_major(300_000, iso::USD));
    }

    // Doc line: "$450,000.00 − $300,000.00 = $150,000.00."
    #[test]
    fn worked_example_net_benefit() {
        let value = net_benefit(
            Money::from_major(450_000, iso::USD),
            Money::from_major(300_000, iso::USD),
        )
        .unwrap();
        assert_eq!(value, Money::from_major(150_000, iso::USD));
    }

    #[test]
    fn total_platform_cost_currency_mismatch_returns_err() {
        assert!(
            total_platform_cost(
                Money::from_major(120_000, iso::USD),
                Money::from_major(80_000, iso::EUR),
                Money::from_major(100_000, iso::USD)
            )
            .is_err()
        );
    }

    #[test]
    fn net_benefit_currency_mismatch_returns_err() {
        assert!(
            net_benefit(
                Money::from_major(450_000, iso::USD),
                Money::from_major(300_000, iso::EUR)
            )
            .is_err()
        );
    }
}

//! # Bed-Day Cost Avoidance
//!
//! Bed-day cost avoidance puts a currency amount on [virtual ward bed
//! days](crate::virtual_ward_bed_days): the cost of the hospital bed days
//! saved by shifting recovery to a virtual ward, netted against the virtual
//! ward's own operating cost.
//!
//! ## How it's calculated
//!
//! ```text
//! Cost avoided = bed days saved × cost per bed day
//!
//! Net savings = cost avoided − virtual ward cost
//! ```
//!
//! ## Why it matters
//!
//! Bed days saved is an operational figure; commissioners decide on money.
//! This converts it into currency and nets off the monitoring, staffing,
//! and equipment cost of the virtual ward, so the case isn't made on gross
//! savings alone.
//!
//! ## Worked example
//!
//! Continuing [virtual ward bed days](crate::virtual_ward_bed_days): 300
//! bed days saved, at an illustrative $500.00 marginal cost per bed day.
//! The virtual ward costs an illustrative $60,000.00 to run over the
//! period.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::bed_day_cost_avoidance::{bed_day_cost_avoided, net_savings};
//!
//! // 300 × $500.00 = $150,000.00.
//! let bed_day_cost_avoided_value = bed_day_cost_avoided(300, Money::from_major(500, iso::USD)).unwrap();
//! assert_eq!(bed_day_cost_avoided_value, Money::from_major(150_000, iso::USD));
//!
//! // $150,000.00 − $60,000.00 = $90,000.00.
//! let net_savings_value = net_savings(Money::from_major(150_000, iso::USD), Money::from_major(60_000, iso::USD)).unwrap();
//! assert_eq!(net_savings_value, Money::from_major(90_000, iso::USD));
//!
//! ```
//!
//! ## Data sources and caveats
//!
//! Use the marginal cost of a bed day — staffing and consumables actually
//! avoided — not the fully allocated average, which includes fixed overhead
//! that doesn't fall when a bed empties. Virtual ward cost should include
//! remote-monitoring devices, clinical staff time, and platform licences.
//!
//! ## Pitfalls
//!
//! - Using the average fully allocated bed-day cost, which overstates the
//!   saving.
//! - Assuming a freed bed generates a saving when it is simply refilled —
//!   the benefit is then capacity, not cash.
//! - Omitting the virtual ward's own cost.
//!
//! ## Sources
//!
//! - NHS England virtual ward guidance and reference costs.
//! - Peer-reviewed health-economic literature on hospital-at-home and
//!   virtual ward cost analysis.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The cost avoided: bed days saved multiplied by the marginal cost of one
/// bed day.
///
/// # Arguments
///
/// * `bed_days_saved` — bed days saved in the period (count).
/// * `cost_per_bed_day` — marginal cost of one hospital bed day.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `cost_per_bed_day`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::bed_day_cost_avoidance::bed_day_cost_avoided;
///
/// // Worked example: 300 × $500.00 = $150,000.00.
/// let value = bed_day_cost_avoided(300, Money::from_major(500, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(150_000, iso::USD));
/// ```
pub fn bed_day_cost_avoided(
    bed_days_saved: u32,
    cost_per_bed_day: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_per_bed_day.mul(bed_days_saved)
}

/// The net savings: cost avoided minus the virtual ward's own operating
/// cost for the period.
///
/// # Arguments
///
/// * `cost_avoided` — bed-day cost avoided, typically the result of
///   [`bed_day_cost_avoided`].
/// * `virtual_ward_cost` — the virtual ward's monitoring, staffing, and
///   equipment cost for the same period.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `cost_avoided` and
/// `virtual_ward_cost` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::bed_day_cost_avoidance::net_savings;
///
/// // Worked example: $150,000.00 − $60,000.00 = $90,000.00.
/// let value = net_savings(Money::from_major(150_000, iso::USD), Money::from_major(60_000, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(90_000, iso::USD));
///
/// assert!(net_savings(Money::from_major(150_000, iso::USD), Money::from_major(60_000, iso::EUR)).is_err());
/// ```
pub fn net_savings(
    cost_avoided: Money<'static, iso::Currency>,
    virtual_ward_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_avoided.sub(virtual_ward_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "300 × $500.00 = $150,000.00."
    #[test]
    fn worked_example_bed_day_cost_avoided() {
        let value = bed_day_cost_avoided(300, Money::from_major(500, iso::USD)).unwrap();
        assert_eq!(value, Money::from_major(150_000, iso::USD));
    }

    // Doc line: "$150,000.00 − $60,000.00 = $90,000.00."
    #[test]
    fn worked_example_net_savings() {
        let value = net_savings(
            Money::from_major(150_000, iso::USD),
            Money::from_major(60_000, iso::USD),
        )
        .unwrap();
        assert_eq!(value, Money::from_major(90_000, iso::USD));
    }

    #[test]
    fn net_savings_currency_mismatch_returns_err() {
        assert!(
            net_savings(
                Money::from_major(150_000, iso::USD),
                Money::from_major(60_000, iso::EUR)
            )
            .is_err()
        );
    }
}

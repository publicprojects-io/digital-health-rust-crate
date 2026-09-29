//! # ED Diversion Cost Avoidance
//!
//! ED diversion cost avoidance puts a currency amount on [ED diversion
//! rate](crate::ed_diversion_rate): the cost of the emergency department
//! visits safely avoided by virtual triage, netted against the triage
//! service's own operating cost.
//!
//! ## How it's calculated
//!
//! ```text
//! Cost avoided = safely diverted visits × avoided cost per ED visit
//!
//! Net savings = cost avoided − triage service cost
//! ```
//!
//! ## Why it matters
//!
//! ED visits are among the costliest low-acuity encounters, so diversion is
//! the headline financial claim for virtual triage. Counting only *safely*
//! diverted visits, as [ED diversion rate](crate::ed_diversion_rate)'s safe
//! diversion rate defines them, keeps the saving from being overstated by
//! returns to the ED.
//!
//! ## Worked example
//!
//! Continuing [ED diversion rate](crate::ed_diversion_rate): 1,150 diverted
//! contacts had no ED return, at an illustrative $800.00 avoided cost per
//! visit. The triage service costs an illustrative $120,000.00 for the
//! period.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::ed_diversion_cost_avoidance::{cost_avoided, net_savings};
//!
//! // 1,150 × $800.00 = $920,000.00.
//! let cost_avoided_value = cost_avoided(1_150, Money::from_major(800, iso::USD)).unwrap();
//! assert_eq!(cost_avoided_value, Money::from_major(920_000, iso::USD));
//!
//! // $920,000.00 − $120,000.00 = $800,000.00.
//! let net_savings_value = net_savings(Money::from_major(920_000, iso::USD), Money::from_major(120_000, iso::USD)).unwrap();
//! assert_eq!(net_savings_value, Money::from_major(800_000, iso::USD));
//!
//! ```
//!
//! ## Data sources and caveats
//!
//! The avoided cost should be the marginal cost or payer payment of the
//! low-acuity ED visit, not a generic average, and should be reduced by the
//! cost of the alternative route the patient took instead (a same-day GP
//! slot, for example).
//!
//! ## Pitfalls
//!
//! - Counting all diverted contacts rather than only safe diversions.
//! - Ignoring the cost of the alternative care route.
//! - Using a high-acuity average ED cost for low-acuity diversions.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on the cost of low-acuity emergency
//!   department attendance.
//! - NHS reference costs for emergency department attendances.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The cost avoided: safely diverted ED visits multiplied by the avoided
/// cost of one visit.
///
/// # Arguments
///
/// * `safely_diverted_visits` — diverted contacts with no ED return in the
///   follow-up window (count).
/// * `avoided_cost_per_visit` — cost avoided per ED visit that did not
///   occur.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_visit`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::ed_diversion_cost_avoidance::cost_avoided;
///
/// // Worked example: 1,150 × $800.00 = $920,000.00.
/// let value = cost_avoided(1_150, Money::from_major(800, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(920_000, iso::USD));
/// ```
pub fn cost_avoided(
    safely_diverted_visits: u32,
    avoided_cost_per_visit: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_visit.mul(safely_diverted_visits)
}

/// The net savings: cost avoided minus the triage service's own operating
/// cost for the period.
///
/// # Arguments
///
/// * `cost_avoided` — ED cost avoided, typically the result of
///   [`cost_avoided`].
/// * `triage_service_cost` — the virtual triage service's licensing,
///   clinical staffing, and administration cost for the same period.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `cost_avoided` and
/// `triage_service_cost` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::ed_diversion_cost_avoidance::net_savings;
///
/// // Worked example: $920,000.00 − $120,000.00 = $800,000.00.
/// let value = net_savings(Money::from_major(920_000, iso::USD), Money::from_major(120_000, iso::USD)).unwrap();
/// assert_eq!(value, Money::from_major(800_000, iso::USD));
///
/// assert!(net_savings(Money::from_major(920_000, iso::USD), Money::from_major(120_000, iso::EUR)).is_err());
/// ```
pub fn net_savings(
    cost_avoided: Money<'static, iso::Currency>,
    triage_service_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_avoided.sub(triage_service_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "1,150 × $800.00 = $920,000.00."
    #[test]
    fn worked_example_cost_avoided() {
        let value = cost_avoided(1_150, Money::from_major(800, iso::USD)).unwrap();
        assert_eq!(value, Money::from_major(920_000, iso::USD));
    }

    // Doc line: "$920,000.00 − $120,000.00 = $800,000.00."
    #[test]
    fn worked_example_net_savings() {
        let value = net_savings(
            Money::from_major(920_000, iso::USD),
            Money::from_major(120_000, iso::USD),
        )
        .unwrap();
        assert_eq!(value, Money::from_major(800_000, iso::USD));
    }

    #[test]
    fn net_savings_currency_mismatch_returns_err() {
        assert!(
            net_savings(
                Money::from_major(920_000, iso::USD),
                Money::from_major(120_000, iso::EUR)
            )
            .is_err()
        );
    }
}

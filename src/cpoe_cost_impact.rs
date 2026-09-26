//! # CPOE Cost Impact
//!
//! CPOE cost impact puts a currency amount on [CPOE adoption
//! rate](crate::cpoe_adoption_rate): the transcription and verification
//! staff cost avoided by each order placed electronically, and the review
//! staff cost incurred by each order given verbally, which requires a
//! mandatory read-back-and-countersign workflow that an electronic order
//! does not.
//!
//! ## How it's calculated
//!
//! ```text
//! Transcription cost avoided = electronic orders × avoided transcription cost per order
//!
//! Verbal order review cost = verbal orders × review cost per order
//! ```
//!
//! ## Why it matters
//!
//! [CPOE adoption rate](crate::cpoe_adoption_rate) already explains the
//! safety case against verbal orders; this module adds the operational
//! staffing cost case. An electronic order removes a manual
//! transcription/verification step entirely, while a verbal order adds a
//! mandatory review step (read-back and countersignature) that a well-run
//! verbal-order safety process requires precisely because it bypasses
//! CPOE's automated checks. Reporting both figures side by side shows that
//! verbal orders are not merely a safety risk but also, independently, the
//! more staff-intensive path.
//!
//! ## Worked example
//!
//! Continuing [CPOE adoption rate](crate::cpoe_adoption_rate)'s worked
//! example: of 10,000 medication orders in a month, 9,400 are placed
//! electronically and 500 are given verbally. An illustrative $1.00
//! transcription cost is avoided per electronic order; an illustrative
//! $15.00 review cost is incurred per verbal order.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::cpoe_cost_impact::{transcription_cost_avoided, verbal_order_review_cost};
//!
//! let avoided_cost_per_order = Money::from_major(1, iso::USD);
//! let review_cost_per_order = Money::from_major(15, iso::USD);
//!
//! // 9,400 × $1.00 = $9,400.00 in avoided transcription cost.
//! let avoided = transcription_cost_avoided(9_400, avoided_cost_per_order).unwrap();
//! assert_eq!(avoided, Money::from_major(9_400, iso::USD));
//!
//! // 500 × $15.00 = $7,500.00 in verbal-order review cost.
//! let review = verbal_order_review_cost(500, review_cost_per_order).unwrap();
//! assert_eq!(review, Money::from_major(7_500, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! Both per-order figures should come from a locally grounded
//! time-and-motion estimate — pharmacy or nursing verification time for
//! electronic orders, and read-back-and-countersign time for verbal
//! orders — multiplied by loaded staff cost, not a generic published
//! figure.
//!
//! ## Pitfalls
//!
//! - **Treating the two figures as directly comparable savings versus
//!   spend** — they represent different things (cost avoided by a
//!   counterfactual manual process, versus cost actually incurred by a
//!   real one) and should not be netted against each other without stating
//!   that explicitly.
//! - **Using one review-cost figure across clinical contexts** — a verbal
//!   order given during a code or emergency has a different review-time
//!   profile than a routine one given because of a system access issue.
//! - **Ignoring order type** — transcription cost avoided varies by order
//!   complexity (a simple refill versus a complex titrated infusion), the
//!   same caveat [CPOE adoption rate](crate::cpoe_adoption_rate) raises
//!   about segmenting by order type generally.
//!
//! ## Sources
//!
//! - Same evidence base as [CPOE adoption
//!   rate](crate::cpoe_adoption_rate): ONC / HealthIT.gov Promoting
//!   Interoperability Program CPOE measures, and Institute for Safe
//!   Medication Practices (ISMP) guidance on verbal orders.
//! - Practice management and pharmacy operations literature on order
//!   transcription and verification time-and-motion studies.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The transcription cost avoided: electronic orders multiplied by the
/// avoided transcription and verification cost of one order.
///
/// # Arguments
///
/// * `electronic_orders` — orders placed directly into the EHR via CPOE in
///   the period (count).
/// * `avoided_cost_per_order` — transcription and verification staff cost
///   avoided per electronic order.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_order`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::cpoe_cost_impact::transcription_cost_avoided;
///
/// // Worked example: 9,400 × $1.00 = $9,400.00.
/// let avoided_cost_per_order = Money::from_major(1, iso::USD);
/// let avoided = transcription_cost_avoided(9_400, avoided_cost_per_order).unwrap();
/// assert_eq!(avoided, Money::from_major(9_400, iso::USD));
/// ```
pub fn transcription_cost_avoided(
    electronic_orders: u32,
    avoided_cost_per_order: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_order.mul(electronic_orders)
}

/// The verbal order review cost: verbal orders multiplied by the
/// read-back-and-countersign review cost of one order.
///
/// # Arguments
///
/// * `verbal_orders` — orders given verbally in the period (count).
/// * `review_cost_per_order` — staff cost of the read-back and
///   countersignature review required for one verbal order.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `review_cost_per_order`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::cpoe_cost_impact::verbal_order_review_cost;
///
/// // Worked example: 500 × $15.00 = $7,500.00.
/// let review_cost_per_order = Money::from_major(15, iso::USD);
/// let review = verbal_order_review_cost(500, review_cost_per_order).unwrap();
/// assert_eq!(review, Money::from_major(7_500, iso::USD));
/// ```
pub fn verbal_order_review_cost(
    verbal_orders: u32,
    review_cost_per_order: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    review_cost_per_order.mul(verbal_orders)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "9,400 × $1.00 = $9,400.00 in avoided transcription cost."
    #[test]
    fn worked_example_transcription_cost_avoided_is_9_400_usd() {
        let avoided_cost_per_order = Money::from_major(1, iso::USD);
        let avoided = transcription_cost_avoided(9_400, avoided_cost_per_order).unwrap();
        assert_eq!(avoided, Money::from_major(9_400, iso::USD));
    }

    // Doc line: "500 × $15.00 = $7,500.00 in verbal-order review cost."
    #[test]
    fn worked_example_verbal_order_review_cost_is_7_500_usd() {
        let review_cost_per_order = Money::from_major(15, iso::USD);
        let review = verbal_order_review_cost(500, review_cost_per_order).unwrap();
        assert_eq!(review, Money::from_major(7_500, iso::USD));
    }
}

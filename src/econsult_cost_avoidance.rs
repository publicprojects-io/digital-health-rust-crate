//! # E-Consult Cost Avoidance
//!
//! E-consult cost avoidance puts a currency amount on [e-consult turnaround
//! time](crate::econsult_turnaround_time): the specialist-visit and patient
//! travel cost avoided by resolving a clinical question through an
//! asynchronous e-consult instead of a full in-person referral, netted
//! against the e-consult platform's own operating cost.
//!
//! ## How it's calculated
//!
//! ```text
//! Cost avoided = e-consults completed × avoided cost per consult
//!
//! Net savings = cost avoided − platform cost
//! ```
//!
//! ## Why it matters
//!
//! Programmes like Champlain BASE eConsult are evaluated in the literature
//! partly on how many in-person referrals they avoid; this module converts
//! that avoided-referral count into the currency terms a commissioning or
//! budget decision is made on, netted against the cost of running the
//! e-consult service itself.
//!
//! ## Worked example
//!
//! Continuing [e-consult turnaround time](crate::econsult_turnaround_time)'s
//! topic: a primary care network completes 400 e-consults in a quarter that
//! would otherwise have required an in-person specialist referral, at an
//! illustrative $120.00 avoided cost per consult (specialist visit cost and
//! patient travel not incurred). The e-consult platform costs an
//! illustrative $10,000.00 for the quarter.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::econsult_cost_avoidance::{cost_avoided, net_savings};
//!
//! let avoided_cost_per_consult = Money::from_major(120, iso::USD);
//!
//! // 400 × $120.00 = $48,000.00.
//! let avoided = cost_avoided(400, avoided_cost_per_consult).unwrap();
//! assert_eq!(avoided, Money::from_major(48_000, iso::USD));
//!
//! // $48,000.00 − $10,000.00 = $38,000.00 net savings for the quarter.
//! let platform_cost = Money::from_major(10_000, iso::USD);
//! let net = net_savings(avoided, platform_cost).unwrap();
//! assert_eq!(net, Money::from_major(38_000, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `avoided_cost_per_consult` should come from a locally grounded estimate
//! of the specific specialist visit type and typical patient travel it
//! replaces, not a generic published figure. `platform_cost` should include
//! specialist time compensation, platform licensing, and administration,
//! amortized over the period measured.
//!
//! ## Pitfalls
//!
//! - **Using one avoided-cost figure across specialties** — a dermatology
//!   e-consult and a cardiology e-consult avoid very different visit costs;
//!   blending them will misstate the case for either.
//! - **Counting every completed e-consult as an avoided referral** — some
//!   e-consults conclude by recommending an in-person referral anyway; only
//!   the share that genuinely substitutes for one should be counted, the
//!   same distinction [e-consult turnaround
//!   time](crate::econsult_turnaround_time)'s own pitfalls raise.
//! - **Treating platform cost as purely variable** — specialist time
//!   compensation is often structured as a retainer or session rate rather
//!   than a strict per-consult cost.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on eConsult services and their effect on
//!   referral volume and cost.
//! - ONC / HealthIT.gov guidance on asynchronous consultation.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The cost avoided: e-consults completed multiplied by the avoided cost of
/// one specialist visit and its associated travel.
///
/// # Arguments
///
/// * `econsults_completed` — e-consults completed in the period that
///   substituted for an in-person referral (count).
/// * `avoided_cost_per_consult` — specialist visit and travel cost avoided
///   per e-consult.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_consult`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::econsult_cost_avoidance::cost_avoided;
///
/// // Worked example: 400 × $120.00 = $48,000.00.
/// let avoided_cost_per_consult = Money::from_major(120, iso::USD);
/// let avoided = cost_avoided(400, avoided_cost_per_consult).unwrap();
/// assert_eq!(avoided, Money::from_major(48_000, iso::USD));
/// ```
pub fn cost_avoided(
    econsults_completed: u32,
    avoided_cost_per_consult: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_consult.mul(econsults_completed)
}

/// The net savings: cost avoided minus the e-consult platform's own
/// operating cost for the period.
///
/// # Arguments
///
/// * `cost_avoided` — specialist visit and travel cost avoided over the
///   period, typically the result of [`cost_avoided`].
/// * `platform_cost` — the e-consult platform's specialist compensation,
///   licensing, and administration cost for the same period.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `cost_avoided` and
/// `platform_cost` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::econsult_cost_avoidance::net_savings;
///
/// // Worked example: $48,000.00 − $10,000.00 = $38,000.00.
/// let avoided = Money::from_major(48_000, iso::USD);
/// let platform_cost = Money::from_major(10_000, iso::USD);
/// let net = net_savings(avoided, platform_cost).unwrap();
/// assert_eq!(net, Money::from_major(38_000, iso::USD));
///
/// let platform_cost_eur = Money::from_major(10_000, iso::EUR);
/// assert!(net_savings(avoided, platform_cost_eur).is_err());
/// ```
pub fn net_savings(
    cost_avoided: Money<'static, iso::Currency>,
    platform_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_avoided.sub(platform_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "400 × $120.00 = $48,000.00."
    #[test]
    fn worked_example_cost_avoided_is_48_000_usd() {
        let avoided_cost_per_consult = Money::from_major(120, iso::USD);
        let avoided = cost_avoided(400, avoided_cost_per_consult).unwrap();
        assert_eq!(avoided, Money::from_major(48_000, iso::USD));
    }

    // Doc line: "$48,000.00 − $10,000.00 = $38,000.00 net savings."
    #[test]
    fn worked_example_net_savings_is_38_000_usd() {
        let avoided_cost_per_consult = Money::from_major(120, iso::USD);
        let avoided = cost_avoided(400, avoided_cost_per_consult).unwrap();
        let platform_cost = Money::from_major(10_000, iso::USD);
        let net = net_savings(avoided, platform_cost).unwrap();
        assert_eq!(net, Money::from_major(38_000, iso::USD));
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let avoided = Money::from_major(48_000, iso::USD);
        let platform_cost_eur = Money::from_major(10_000, iso::EUR);
        assert!(net_savings(avoided, platform_cost_eur).is_err());
    }
}

//! # Telehealth Cost Avoidance
//!
//! Telehealth cost avoidance puts a currency amount on [telehealth visit
//! rate](crate::telehealth_visit_rate): the travel and facility cost avoided
//! by delivering a visit remotely instead of in person, netted against the
//! telehealth platform's own operating cost. [Telehealth visit
//! rate](crate::telehealth_visit_rate)'s rustdoc says the "right" rate is a
//! clinical and operational judgement, not a target to maximize; this
//! module supplies the other half of that judgement — the actual currency
//! magnitude a business case or reimbursement policy discussion is decided
//! on.
//!
//! ## How it's calculated
//!
//! ```text
//! Cost avoided = telehealth encounters × avoided cost per encounter
//!
//! Net savings = cost avoided − platform cost
//! ```
//!
//! ## Why it matters
//!
//! "Avoided cost" here is deliberately broader than provider revenue: it
//! includes patient travel time and expense and clinic overhead not
//! incurred, value that accrues to the patient and payer side as much as to
//! the provider. Reporting cost avoided alone overstates a telehealth
//! programme's case, though, because the programme itself has a cost — a
//! platform licence, technical support, training — that has to be netted
//! out before the figure means anything for a budget decision.
//!
//! ## Worked example
//!
//! Continuing [telehealth visit rate](crate::telehealth_visit_rate)'s
//! worked example: a community mental health service delivers 1,600 video
//! encounters in a quarter, at an illustrative $40.00 avoided cost per
//! encounter (patient travel and clinic overhead not incurred). The
//! telehealth platform costs an illustrative $18,000.00 for the quarter.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::telehealth_cost_avoidance::{cost_avoided, net_savings};
//!
//! let avoided_cost_per_encounter = Money::from_major(40, iso::USD);
//!
//! // 1,600 × $40.00 = $64,000.00.
//! let avoided = cost_avoided(1_600, avoided_cost_per_encounter).unwrap();
//! assert_eq!(avoided, Money::from_major(64_000, iso::USD));
//!
//! // $64,000.00 − $18,000.00 = $46,000.00 net savings for the quarter.
//! let platform_cost = Money::from_major(18_000, iso::USD);
//! let net = net_savings(avoided, platform_cost).unwrap();
//! assert_eq!(net, Money::from_major(46_000, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `avoided_cost_per_encounter` should come from a locally grounded
//! estimate of patient travel distance/time and facility overhead for the
//! specific service line being measured, not a generic published figure —
//! travel burden varies enormously by geography and patient population.
//! `platform_cost` should include licensing, support, and training,
//! amortized over the period being measured.
//!
//! ## Pitfalls
//!
//! - **Using one avoided-cost figure across specialties or geographies** —
//!   a rural service with long average travel distances has a very
//!   different avoided cost than an urban one; blending them will misstate
//!   the case for either.
//! - **Treating platform cost as purely variable** — much of it (licensing,
//!   training) is a fixed or step cost, not linear per encounter; comparing
//!   net savings at low volume against a programme's early-stage
//!   fixed costs will make a young programme look worse than its
//!   trajectory actually is.
//! - **Adding cost avoidance to provider revenue** — this module measures
//!   value created for the patient/payer side, not new cash revenue for the
//!   provider organization; see [no-show lost
//!   revenue](crate::no_show_lost_revenue) for a genuinely revenue-side
//!   figure, and don't sum the two without being explicit about which is
//!   which.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on telehealth cost-effectiveness and patient
//!   travel-cost avoidance.
//! - CMS / ONC telehealth cost and utilization policy publications.
//! - Healthcare economics literature on total cost of care and virtual care
//!   substitution.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The cost avoided: telehealth encounters multiplied by the avoided cost of
/// one encounter.
///
/// # Arguments
///
/// * `telehealth_encounters` — completed telehealth encounters in the
///   period (count).
/// * `avoided_cost_per_encounter` — travel and facility cost avoided by
///   delivering one encounter remotely instead of in person.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_encounter`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::telehealth_cost_avoidance::cost_avoided;
///
/// // Worked example: 1,600 × $40.00 = $64,000.00.
/// let avoided_cost_per_encounter = Money::from_major(40, iso::USD);
/// let avoided = cost_avoided(1_600, avoided_cost_per_encounter).unwrap();
/// assert_eq!(avoided, Money::from_major(64_000, iso::USD));
/// ```
pub fn cost_avoided(
    telehealth_encounters: u32,
    avoided_cost_per_encounter: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_encounter.mul(telehealth_encounters)
}

/// The net savings: cost avoided minus the telehealth platform's own
/// operating cost for the period.
///
/// # Arguments
///
/// * `cost_avoided` — travel and facility cost avoided over the period,
///   typically the result of [`cost_avoided`].
/// * `platform_cost` — the telehealth platform's licensing, support, and
///   training cost for the same period.
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
/// use digital_health::telehealth_cost_avoidance::net_savings;
///
/// // Worked example: $64,000.00 − $18,000.00 = $46,000.00.
/// let avoided = Money::from_major(64_000, iso::USD);
/// let platform_cost = Money::from_major(18_000, iso::USD);
/// let net = net_savings(avoided, platform_cost).unwrap();
/// assert_eq!(net, Money::from_major(46_000, iso::USD));
///
/// let platform_cost_eur = Money::from_major(18_000, iso::EUR);
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

    // Doc line: "1,600 × $40.00 = $64,000.00."
    #[test]
    fn worked_example_cost_avoided_is_64_000_usd() {
        let avoided_cost_per_encounter = Money::from_major(40, iso::USD);
        let avoided = cost_avoided(1_600, avoided_cost_per_encounter).unwrap();
        assert_eq!(avoided, Money::from_major(64_000, iso::USD));
    }

    // Doc line: "$64,000.00 − $18,000.00 = $46,000.00 net savings."
    #[test]
    fn worked_example_net_savings_is_46_000_usd() {
        let avoided_cost_per_encounter = Money::from_major(40, iso::USD);
        let avoided = cost_avoided(1_600, avoided_cost_per_encounter).unwrap();
        let platform_cost = Money::from_major(18_000, iso::USD);
        let net = net_savings(avoided, platform_cost).unwrap();
        assert_eq!(net, Money::from_major(46_000, iso::USD));
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let avoided = Money::from_major(64_000, iso::USD);
        let platform_cost_eur = Money::from_major(18_000, iso::EUR);
        assert!(net_savings(avoided, platform_cost_eur).is_err());
    }
}

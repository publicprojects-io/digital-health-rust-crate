//! # Patient Self-Scheduling Cost Savings
//!
//! Patient self-scheduling cost savings puts a currency amount on [patient
//! self-scheduling rate](crate::patient_self_scheduling_rate): the staff
//! phone-scheduling time avoided by each appointment a patient books
//! directly online, netted against the scheduling platform's own operating
//! cost. Adoption of self-scheduling alone doesn't tell a finance team
//! whether the platform investment pays off; this module supplies the
//! currency figure that decision is actually made on.
//!
//! ## How it's calculated
//!
//! ```text
//! Staff time saved cost = self-scheduled appointments × avoided staff cost per booking
//!
//! Net savings = staff time saved cost − platform cost
//! ```
//!
//! ## Why it matters
//!
//! Every appointment a patient books online is, from a staffing point of
//! view, a phone call that a scheduling coordinator didn't have to answer —
//! but the platform that enables that also has a running cost of its own
//! (licensing, support, training), which has to be netted out before the
//! figure means anything for a budget decision, the same caution [telehealth
//! cost avoidance](crate::telehealth_cost_avoidance) makes about its own
//! platform cost.
//!
//! ## Worked example
//!
//! Continuing [patient self-scheduling
//! rate](crate::patient_self_scheduling_rate)'s worked example: a community
//! clinic has 640 appointments self-scheduled online in a month, at an
//! illustrative $6.00 avoided staff cost per booking (average staff time ×
//! loaded hourly cost for the phone call self-scheduling replaced). The
//! scheduling platform costs an illustrative $900.00 for the month.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::patient_self_scheduling_cost_savings::{staff_time_saved_cost, net_savings};
//!
//! let avoided_cost_per_booking = Money::from_major(6, iso::USD);
//!
//! // 640 × $6.00 = $3,840.00.
//! let saved = staff_time_saved_cost(640, avoided_cost_per_booking).unwrap();
//! assert_eq!(saved, Money::from_major(3_840, iso::USD));
//!
//! // $3,840.00 − $900.00 = $2,940.00 net savings for the month.
//! let platform_cost = Money::from_major(900, iso::USD);
//! let net = net_savings(saved, platform_cost).unwrap();
//! assert_eq!(net, Money::from_major(2_940, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `avoided_cost_per_booking` should come from a locally grounded
//! time-and-motion estimate of scheduling-call handling time multiplied by
//! loaded staff cost, not a generic published figure — call-handling time
//! varies by visit type and organization. `platform_cost` should include
//! licensing, support, and training, amortized over the period measured.
//!
//! ## Pitfalls
//!
//! - **Using one avoided-cost figure across visit types** — a complex
//!   new-patient booking call takes far longer to handle than a routine
//!   follow-up; blending them will misstate the case for either.
//! - **Treating platform cost as purely variable** — much of it (licensing,
//!   training) is fixed or step cost, not linear per booking; comparing net
//!   savings at low adoption against a programme's early fixed costs will
//!   make a young rollout look worse than its trajectory actually is.
//! - **Assuming saved staff time converts directly to reduced headcount
//!   cost** — time saved on scheduling calls is only a cash saving if it is
//!   actually redeployed or reduces staffing need; otherwise it is capacity
//!   freed for other work, not money saved.
//!
//! ## Sources
//!
//! - Practice management and call-center literature on scheduling-staff
//!   time-and-motion studies.
//! - ONC / HealthIT.gov digital front-door cost-effectiveness discussions.
//! - Healthcare operations literature on self-service technology return on
//!   investment.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The staff time saved cost: self-scheduled appointments multiplied by the
/// avoided staff cost of one phone-scheduled booking.
///
/// # Arguments
///
/// * `self_scheduled` — appointments booked by the patient directly online
///   in the period (count).
/// * `avoided_cost_per_booking` — staff time cost avoided per booking that
///   self-scheduling replaced.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_booking`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::patient_self_scheduling_cost_savings::staff_time_saved_cost;
///
/// // Worked example: 640 × $6.00 = $3,840.00.
/// let avoided_cost_per_booking = Money::from_major(6, iso::USD);
/// let saved = staff_time_saved_cost(640, avoided_cost_per_booking).unwrap();
/// assert_eq!(saved, Money::from_major(3_840, iso::USD));
/// ```
pub fn staff_time_saved_cost(
    self_scheduled: u32,
    avoided_cost_per_booking: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_booking.mul(self_scheduled)
}

/// The net savings: staff time saved cost minus the scheduling platform's
/// own operating cost for the period.
///
/// # Arguments
///
/// * `staff_time_saved_cost` — staff time cost avoided over the period,
///   typically the result of [`staff_time_saved_cost`].
/// * `platform_cost` — the scheduling platform's licensing, support, and
///   training cost for the same period.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `staff_time_saved_cost` and
/// `platform_cost` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::patient_self_scheduling_cost_savings::net_savings;
///
/// // Worked example: $3,840.00 − $900.00 = $2,940.00.
/// let saved = Money::from_major(3_840, iso::USD);
/// let platform_cost = Money::from_major(900, iso::USD);
/// let net = net_savings(saved, platform_cost).unwrap();
/// assert_eq!(net, Money::from_major(2_940, iso::USD));
///
/// let platform_cost_eur = Money::from_major(900, iso::EUR);
/// assert!(net_savings(saved, platform_cost_eur).is_err());
/// ```
pub fn net_savings(
    staff_time_saved_cost: Money<'static, iso::Currency>,
    platform_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    staff_time_saved_cost.sub(platform_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "640 × $6.00 = $3,840.00."
    #[test]
    fn worked_example_staff_time_saved_cost_is_3_840_usd() {
        let avoided_cost_per_booking = Money::from_major(6, iso::USD);
        let saved = staff_time_saved_cost(640, avoided_cost_per_booking).unwrap();
        assert_eq!(saved, Money::from_major(3_840, iso::USD));
    }

    // Doc line: "$3,840.00 − $900.00 = $2,940.00 net savings."
    #[test]
    fn worked_example_net_savings_is_2_940_usd() {
        let avoided_cost_per_booking = Money::from_major(6, iso::USD);
        let saved = staff_time_saved_cost(640, avoided_cost_per_booking).unwrap();
        let platform_cost = Money::from_major(900, iso::USD);
        let net = net_savings(saved, platform_cost).unwrap();
        assert_eq!(net, Money::from_major(2_940, iso::USD));
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let saved = Money::from_major(3_840, iso::USD);
        let platform_cost_eur = Money::from_major(900, iso::EUR);
        assert!(net_savings(saved, platform_cost_eur).is_err());
    }
}

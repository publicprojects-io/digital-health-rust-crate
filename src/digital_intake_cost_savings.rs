//! # Digital Intake Cost Savings
//!
//! Digital intake cost savings puts a currency amount on [digital intake
//! form completion rate](crate::digital_intake_form_completion_rate): the
//! staff data-entry time avoided by each form a patient completes online
//! instead of on paper, netted against the intake platform's own operating
//! cost.
//!
//! ## How it's calculated
//!
//! ```text
//! Staff time saved cost = forms completed online × avoided cost per form
//!
//! Net savings = staff time saved cost − platform cost
//! ```
//!
//! ## Why it matters
//!
//! Every paper intake form a patient does not fill out is, from a staffing
//! point of view, data a front-desk or medical-records employee did not
//! have to key into the EHR by hand — but the intake platform that enables
//! that has its own running cost, which must be netted out before the
//! figure means anything for a budget decision, the same caution [telehealth
//! cost avoidance](crate::telehealth_cost_avoidance) and [patient
//! self-scheduling cost
//! savings](crate::patient_self_scheduling_cost_savings) make about their
//! own platform costs.
//!
//! ## Worked example
//!
//! Continuing [digital intake form completion
//! rate](crate::digital_intake_form_completion_rate)'s worked example: a
//! clinic has 1,200 intake forms completed online in a month, at an
//! illustrative $3.50 avoided staff data-entry cost per form. The intake
//! platform costs an illustrative $600.00 for the month.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::digital_intake_cost_savings::{staff_time_saved_cost, net_savings};
//!
//! let avoided_cost_per_form = Money::from_str("3.50", iso::USD).unwrap();
//!
//! // 1,200 × $3.50 = $4,200.00.
//! let saved = staff_time_saved_cost(1_200, avoided_cost_per_form).unwrap();
//! assert_eq!(saved, Money::from_major(4_200, iso::USD));
//!
//! // $4,200.00 − $600.00 = $3,600.00 net savings for the month.
//! let platform_cost = Money::from_major(600, iso::USD);
//! let net = net_savings(saved, platform_cost).unwrap();
//! assert_eq!(net, Money::from_major(3_600, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `avoided_cost_per_form` should come from a locally grounded
//! time-and-motion estimate of manual data-entry time multiplied by loaded
//! staff cost, not a generic published figure. `platform_cost` should
//! include licensing, support, and integration cost, amortized over the
//! period measured.
//!
//! ## Pitfalls
//!
//! - **Using one avoided-cost figure across visit types** — a
//!   new-patient intake form takes far longer to key in manually than a
//!   routine follow-up update; blending them will misstate the case for
//!   either.
//! - **Counting a form that still requires manual review** — if staff still
//!   have to review and correct every digitally submitted form before it's
//!   usable, the time saved is partial, not complete.
//! - **Treating platform cost as purely variable** — much of it (licensing,
//!   integration) is fixed or step cost, not linear per form.
//!
//! ## Sources
//!
//! - Practice management literature on registration and data-entry
//!   time-and-motion studies.
//! - ONC / HealthIT.gov digital front-door cost-effectiveness discussions.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The staff time saved cost: forms completed online multiplied by the
/// avoided staff cost of one manually keyed-in form.
///
/// # Arguments
///
/// * `forms_completed_online` — intake forms completed by the patient
///   online in the period (count).
/// * `avoided_cost_per_form` — staff data-entry cost avoided per form
///   completed online instead of on paper.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `avoided_cost_per_form`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::digital_intake_cost_savings::staff_time_saved_cost;
///
/// // Worked example: 1,200 × $3.50 = $4,200.00.
/// let avoided_cost_per_form = Money::from_str("3.50", iso::USD).unwrap();
/// let saved = staff_time_saved_cost(1_200, avoided_cost_per_form).unwrap();
/// assert_eq!(saved, Money::from_major(4_200, iso::USD));
/// ```
pub fn staff_time_saved_cost(
    forms_completed_online: u32,
    avoided_cost_per_form: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    avoided_cost_per_form.mul(forms_completed_online)
}

/// The net savings: staff time saved cost minus the intake platform's own
/// operating cost for the period.
///
/// # Arguments
///
/// * `staff_time_saved_cost` — staff data-entry cost avoided over the
///   period, typically the result of [`staff_time_saved_cost`].
/// * `platform_cost` — the intake platform's licensing, support, and
///   integration cost for the same period.
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
/// use digital_health::digital_intake_cost_savings::net_savings;
///
/// // Worked example: $4,200.00 − $600.00 = $3,600.00.
/// let saved = Money::from_major(4_200, iso::USD);
/// let platform_cost = Money::from_major(600, iso::USD);
/// let net = net_savings(saved, platform_cost).unwrap();
/// assert_eq!(net, Money::from_major(3_600, iso::USD));
///
/// let platform_cost_eur = Money::from_major(600, iso::EUR);
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

    // Doc line: "1,200 × $3.50 = $4,200.00."
    #[test]
    fn worked_example_staff_time_saved_cost_is_4_200_usd() {
        let avoided_cost_per_form = Money::from_str("3.50", iso::USD).unwrap();
        let saved = staff_time_saved_cost(1_200, avoided_cost_per_form).unwrap();
        assert_eq!(saved, Money::from_major(4_200, iso::USD));
    }

    // Doc line: "$4,200.00 − $600.00 = $3,600.00 net savings."
    #[test]
    fn worked_example_net_savings_is_3_600_usd() {
        let avoided_cost_per_form = Money::from_str("3.50", iso::USD).unwrap();
        let saved = staff_time_saved_cost(1_200, avoided_cost_per_form).unwrap();
        let platform_cost = Money::from_major(600, iso::USD);
        let net = net_savings(saved, platform_cost).unwrap();
        assert_eq!(net, Money::from_major(3_600, iso::USD));
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let saved = Money::from_major(4_200, iso::USD);
        let platform_cost_eur = Money::from_major(600, iso::EUR);
        assert!(net_savings(saved, platform_cost_eur).is_err());
    }
}

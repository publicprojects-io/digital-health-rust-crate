//! # Patient Acquisition Cost
//!
//! True patient acquisition cost (CAC) is the fully loaded cost of winning
//! new patients: not just ad spend, but agency fees, the technology stack
//! that supports acquisition, and the clinical intake labour spent
//! onboarding each one. This module puts that on `Money` and divides it by
//! the new patients acquired; [patient acquisition
//! efficiency](crate::patient_acquisition_efficiency) then compares it to
//! lifetime value.
//!
//! ## How it's calculated
//!
//! ```text
//! Total acquisition cost = ad spend + agency fees + technology cost + intake labour cost
//!
//! Cost per new patient = total acquisition cost / new patients
//! ```
//!
//! ## Why it matters
//!
//! CAC built from ad-platform spend alone is routinely undercounted, which
//! makes every downstream ratio look better than it is. Summing all four
//! components, in one currency, gives the figure a growth decision should
//! actually be made on.
//!
//! ## Worked example
//!
//! In a quarter a digital weight-management programme spends an
//! illustrative $40,000.00 on ads, $8,000.00 in agency fees, $6,000.00 on
//! acquisition technology, and $16,000.00 of clinical intake labour, and
//! acquires 140 new patients.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::patient_acquisition_cost::{cost_per_new_patient, total_acquisition_cost};
//!
//! // $40,000 + $8,000 + $6,000 + $16,000 = $70,000.00.
//! let total = total_acquisition_cost(
//!     Money::from_major(40_000, iso::USD),
//!     Money::from_major(8_000, iso::USD),
//!     Money::from_major(6_000, iso::USD),
//!     Money::from_major(16_000, iso::USD),
//! )
//! .unwrap();
//! assert_eq!(total, Money::from_major(70_000, iso::USD));
//!
//! // $70,000.00 / 140 = $500.00 per new patient.
//! let cac = cost_per_new_patient(total, 140).unwrap();
//! assert_eq!(cac, Money::from_major(500, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! Costs come from finance and payroll, not ad dashboards; intake labour is
//! staff time spent on new-patient onboarding, at a loaded hourly rate. All
//! amounts must be in one currency and cover the same period as
//! `new_patients`.
//!
//! ## Pitfalls
//!
//! - **Leaving out a component** — agency fees and intake labour are the
//!   ones most often missed.
//! - **Counting patients who never start** — `new_patients` should be
//!   those who completed onboarding, on the same definition every period.
//! - **Dividing by zero patients** — a period with spend but no
//!   acquisitions returns [`MoneyError::DivisionByZero`], not a zero cost.
//!
//! ## Sources
//!
//! - Standard customer-acquisition-cost definitions from
//!   subscription-business unit-economics literature.
//! - Healthcare marketing literature on fully loaded patient acquisition
//!   cost.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The total acquisition cost: ad spend, agency fees, technology cost, and
/// clinical intake labour cost summed.
///
/// # Arguments
///
/// * `ad_spend` — paid media spend.
/// * `agency_fees` — agency and contractor fees.
/// * `technology_cost` — cost of the acquisition technology stack.
/// * `intake_labor_cost` — clinical intake labour spent onboarding new
///   patients.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if the four arguments are not
/// all denominated in the same currency, or [`MoneyError::Overflow`] if the
/// sum overflows.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::patient_acquisition_cost::total_acquisition_cost;
///
/// // Worked example: $40,000 + $8,000 + $6,000 + $16,000 = $70,000.00.
/// let total = total_acquisition_cost(
///     Money::from_major(40_000, iso::USD),
///     Money::from_major(8_000, iso::USD),
///     Money::from_major(6_000, iso::USD),
///     Money::from_major(16_000, iso::USD),
/// )
/// .unwrap();
/// assert_eq!(total, Money::from_major(70_000, iso::USD));
/// ```
pub fn total_acquisition_cost(
    ad_spend: Money<'static, iso::Currency>,
    agency_fees: Money<'static, iso::Currency>,
    technology_cost: Money<'static, iso::Currency>,
    intake_labor_cost: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    ad_spend
        .add(agency_fees)?
        .add(technology_cost)?
        .add(intake_labor_cost)
}

/// The cost per new patient: total acquisition cost divided by the number
/// of new patients acquired.
///
/// # Arguments
///
/// * `total_acquisition_cost` — fully loaded acquisition cost for the
///   period, typically the result of [`total_acquisition_cost`].
/// * `new_patients` — new patients acquired in the period (count).
///
/// # Errors
///
/// Returns [`MoneyError::DivisionByZero`] if `new_patients` is zero, or
/// [`MoneyError::Overflow`] if the division overflows.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::patient_acquisition_cost::cost_per_new_patient;
///
/// // Worked example: $70,000.00 / 140 = $500.00.
/// let cac = cost_per_new_patient(Money::from_major(70_000, iso::USD), 140).unwrap();
/// assert_eq!(cac, Money::from_major(500, iso::USD));
///
/// assert!(cost_per_new_patient(Money::from_major(70_000, iso::USD), 0).is_err());
/// ```
pub fn cost_per_new_patient(
    total_acquisition_cost: Money<'static, iso::Currency>,
    new_patients: u32,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    total_acquisition_cost.div(new_patients)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worked_example_total() -> Money<'static, iso::Currency> {
        total_acquisition_cost(
            Money::from_major(40_000, iso::USD),
            Money::from_major(8_000, iso::USD),
            Money::from_major(6_000, iso::USD),
            Money::from_major(16_000, iso::USD),
        )
        .unwrap()
    }

    // Doc line: "$40,000 + $8,000 + $6,000 + $16,000 = $70,000.00."
    #[test]
    fn worked_example_total_acquisition_cost_is_70_000_usd() {
        assert_eq!(worked_example_total(), Money::from_major(70_000, iso::USD));
    }

    // Doc line: "$70,000.00 / 140 = $500.00 per new patient."
    #[test]
    fn worked_example_cost_per_new_patient_is_500_usd() {
        let cac = cost_per_new_patient(worked_example_total(), 140).unwrap();
        assert_eq!(cac, Money::from_major(500, iso::USD));
    }

    #[test]
    fn zero_new_patients_returns_err() {
        assert!(cost_per_new_patient(worked_example_total(), 0).is_err());
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let result = total_acquisition_cost(
            Money::from_major(40_000, iso::USD),
            Money::from_major(8_000, iso::EUR),
            Money::from_major(6_000, iso::USD),
            Money::from_major(16_000, iso::USD),
        );
        assert!(result.is_err());
    }
}

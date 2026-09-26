//! # Remote Patient Monitoring Billing Revenue
//!
//! Remote patient monitoring billing revenue puts a currency amount on
//! [remote patient monitoring adherence
//! rate](crate::remote_patient_monitoring_adherence_rate)'s cohort
//! billing-eligible rate: the revenue actually billable for the patients
//! who met the period's monitoring-day threshold, and the revenue put at
//! risk by the patients who didn't.
//!
//! ## How it's calculated
//!
//! ```text
//! Billable revenue = billing-eligible patients × reimbursement per patient
//!
//! Revenue at risk = non-eligible patients × reimbursement per patient
//! ```
//!
//! ## Why it matters
//!
//! [Remote patient monitoring adherence
//! rate](crate::remote_patient_monitoring_adherence_rate) already explains
//! why the billing-day threshold matters clinically and operationally; this
//! module converts the cohort split into the currency terms a programme's
//! finance case is built on. "Revenue at risk" is the more actionable
//! number of the two for programme managers: it quantifies exactly how much
//! is being left on the table by patients who didn't clear the threshold
//! this period, which is what an adherence-improvement intervention is
//! justified against.
//!
//! ## Worked example
//!
//! Continuing [remote patient monitoring adherence
//! rate](crate::remote_patient_monitoring_adherence_rate)'s worked example:
//! a cardiology programme enrols 400 patients; 280 meet the period's
//! billing-day threshold and 120 don't, at an illustrative $50.00
//! per-patient reimbursement (the actual rate depends on the specific CPT
//! code, payer, and year, and should be looked up rather than assumed).
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::remote_patient_monitoring_billing_revenue::{
//!     billable_revenue, revenue_at_risk,
//! };
//!
//! let reimbursement_per_patient = Money::from_major(50, iso::USD);
//!
//! // 280 × $50.00 = $14,000.00 billable this period.
//! let billable = billable_revenue(280, reimbursement_per_patient).unwrap();
//! assert_eq!(billable, Money::from_major(14_000, iso::USD));
//!
//! // 120 × $50.00 = $6,000.00 at risk from patients below the threshold.
//! let at_risk = revenue_at_risk(120, reimbursement_per_patient).unwrap();
//! assert_eq!(at_risk, Money::from_major(6_000, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `reimbursement_per_patient` should be looked up from the payer's current
//! fee schedule for the specific CPT/HCPCS code being billed (e.g. CMS's
//! remote physiologic or remote therapeutic monitoring codes) — it changes
//! annually and varies by locality and payer, so it should never be
//! hard-coded as a constant in calling code.
//!
//! ## Pitfalls
//!
//! - **Assuming one reimbursement figure covers every billed code** — a
//!   monitoring programme typically bills multiple codes in the same period
//!   (device supply, device setup, and clinical-time codes each have
//!   separate reimbursement), and blending them into a single
//!   per-patient figure will misstate both `billable_revenue` and
//!   `revenue_at_risk`.
//! - **Treating revenue at risk as revenue that will be recovered by
//!   intervention** — some non-eligible patients will never reach the
//!   threshold regardless of outreach (e.g. a patient who is hospitalized
//!   for part of the period); revenue at risk is an upper bound on
//!   recoverable revenue, not a forecast.
//! - **Using a stale reimbursement figure** — payer fee schedules update at
//!   least annually; a figure more than a year old should be re-verified
//!   before it drives a business case.
//!
//! ## Sources
//!
//! - See [remote patient monitoring adherence
//!   rate](crate::remote_patient_monitoring_adherence_rate)'s `## Sources`
//!   for the underlying billing-threshold evidence base; this module adds
//!   only the currency arithmetic on top.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{FormattableCurrency, Money, MoneyError};

/// The billable revenue: patients who met the period's billing-day
/// threshold, multiplied by the reimbursement for one patient.
///
/// # Arguments
///
/// * `billing_eligible_patients` — enrolled patients whose monitoring days
///   met the payer's threshold for the period (count).
/// * `reimbursement_per_patient` — the payer's reimbursement for one
///   patient's billable period, for the specific code being calculated.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `reimbursement_per_patient`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::remote_patient_monitoring_billing_revenue::billable_revenue;
///
/// // Worked example: 280 × $50.00 = $14,000.00.
/// let reimbursement_per_patient = Money::from_major(50, iso::USD);
/// let billable = billable_revenue(280, reimbursement_per_patient).unwrap();
/// assert_eq!(billable, Money::from_major(14_000, iso::USD));
/// ```
pub fn billable_revenue<T: FormattableCurrency>(
    billing_eligible_patients: u32,
    reimbursement_per_patient: Money<'_, T>,
) -> Result<Money<'_, T>, MoneyError> {
    reimbursement_per_patient.mul(billing_eligible_patients)
}

/// The revenue at risk: patients who did not meet the period's billing-day
/// threshold, multiplied by the reimbursement for one patient.
///
/// # Arguments
///
/// * `non_eligible_patients` — enrolled patients whose monitoring days fell
///   short of the payer's threshold for the period (count).
/// * `reimbursement_per_patient` — the payer's reimbursement for one
///   patient's billable period, for the specific code being calculated.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `reimbursement_per_patient`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::remote_patient_monitoring_billing_revenue::revenue_at_risk;
///
/// // Worked example: 120 × $50.00 = $6,000.00.
/// let reimbursement_per_patient = Money::from_major(50, iso::USD);
/// let at_risk = revenue_at_risk(120, reimbursement_per_patient).unwrap();
/// assert_eq!(at_risk, Money::from_major(6_000, iso::USD));
/// ```
pub fn revenue_at_risk<T: FormattableCurrency>(
    non_eligible_patients: u32,
    reimbursement_per_patient: Money<'_, T>,
) -> Result<Money<'_, T>, MoneyError> {
    reimbursement_per_patient.mul(non_eligible_patients)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_money::iso;

    // Doc line: "280 × $50.00 = $14,000.00 billable this period."
    #[test]
    fn worked_example_billable_revenue_is_14_000_usd() {
        let reimbursement_per_patient = Money::from_major(50, iso::USD);
        let billable = billable_revenue(280, reimbursement_per_patient).unwrap();
        assert_eq!(billable, Money::from_major(14_000, iso::USD));
    }

    // Doc line: "120 × $50.00 = $6,000.00 at risk from patients below the
    // threshold."
    #[test]
    fn worked_example_revenue_at_risk_is_6_000_usd() {
        let reimbursement_per_patient = Money::from_major(50, iso::USD);
        let at_risk = revenue_at_risk(120, reimbursement_per_patient).unwrap();
        assert_eq!(at_risk, Money::from_major(6_000, iso::USD));
    }

    #[test]
    fn billable_and_at_risk_together_cover_the_full_cohort() {
        let reimbursement_per_patient = Money::from_major(50, iso::USD);
        let billable = billable_revenue(280, reimbursement_per_patient).unwrap();
        let at_risk = revenue_at_risk(120, reimbursement_per_patient).unwrap();
        let full_cohort_at_rate = reimbursement_per_patient.mul(400_u32).unwrap();
        assert_eq!(billable.add(at_risk).unwrap(), full_cohort_at_rate);
    }
}

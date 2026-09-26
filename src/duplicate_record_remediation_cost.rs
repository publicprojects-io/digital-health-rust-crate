//! # Duplicate Record Remediation Cost
//!
//! Duplicate record remediation cost puts a currency amount on [duplicate
//! patient record rate](crate::duplicate_patient_record_rate): the health
//! information management staff cost already spent resolving identified
//! duplicates, and the outstanding cost exposure represented by duplicates
//! not yet resolved.
//!
//! ## How it's calculated
//!
//! ```text
//! Remediation cost = duplicates resolved × cost per resolution
//!
//! Backlog cost = duplicates unresolved × cost per resolution
//! ```
//!
//! ## Why it matters
//!
//! [Duplicate patient record rate](crate::duplicate_patient_record_rate)
//! already explains why an unresolved duplicate is a patient-safety risk;
//! this module converts the resolved/unresolved cohort split into the
//! currency terms a health information management (HIM) department's
//! staffing case is built on. Backlog cost is the more actionable of the
//! two figures for planning purposes: it quantifies the remaining
//! investment needed to close out known risk, the same role [remote patient
//! monitoring billing
//! revenue](crate::remote_patient_monitoring_billing_revenue)'s "revenue at
//! risk" plays for its own cohort split.
//!
//! ## Worked example
//!
//! Continuing [duplicate patient record
//! rate](crate::duplicate_patient_record_rate)'s worked example: of the
//! 42,000 identified duplicates, 31,500 have been resolved, leaving 10,500
//! unresolved, at an illustrative $18.00 HIM staff cost per resolution.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::duplicate_record_remediation_cost::{remediation_cost, backlog_cost};
//!
//! let cost_per_resolution = Money::from_major(18, iso::USD);
//!
//! // 31,500 × $18.00 = $567,000.00 already spent resolving duplicates.
//! let spent = remediation_cost(31_500, cost_per_resolution).unwrap();
//! assert_eq!(spent, Money::from_major(567_000, iso::USD));
//!
//! // 10,500 × $18.00 = $189,000.00 in outstanding backlog cost.
//! let backlog = backlog_cost(10_500, cost_per_resolution).unwrap();
//! assert_eq!(backlog, Money::from_major(189_000, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `cost_per_resolution` should come from a locally grounded time-and-motion
//! estimate of HIM staff time to research, verify, and merge one duplicate
//! record, multiplied by loaded staff cost — resolution complexity (and
//! therefore cost) varies with how much clinical documentation has
//! accumulated on each duplicate chart.
//!
//! ## Pitfalls
//!
//! - **Using one resolution-cost figure regardless of record age or
//!   complexity** — a duplicate identified and merged quickly after
//!   creation costs far less to resolve than one with years of accumulated,
//!   fragmented documentation.
//! - **Treating backlog cost as a one-time figure** — new duplicates are
//!   continuously created by ongoing registration activity; backlog cost
//!   should be remeasured on the same cadence as [duplicate patient record
//!   rate](crate::duplicate_patient_record_rate) itself, not calculated once.
//! - **Ignoring the cost of not resolving** — backlog cost is a staffing
//!   investment figure, not the (much harder to quantify, and not modelled
//!   by this crate) cost of a clinical error caused by an unresolved
//!   duplicate; the two should not be conflated.
//!
//! ## Sources
//!
//! - AHIMA (American Health Information Management Association), guidance
//!   on managing duplicate health records and remediation workflows.
//! - Same evidence base as [duplicate patient record
//!   rate](crate::duplicate_patient_record_rate): ONC / HealthIT.gov and Pew
//!   Charitable Trusts research on patient identity matching.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The remediation cost: duplicates resolved multiplied by the cost of one
/// resolution.
///
/// # Arguments
///
/// * `duplicates_resolved` — identified duplicates reviewed and merged in
///   the period (count).
/// * `cost_per_resolution` — health information management staff cost to
///   research, verify, and merge one duplicate record.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `cost_per_resolution`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::duplicate_record_remediation_cost::remediation_cost;
///
/// // Worked example: 31,500 × $18.00 = $567,000.00.
/// let cost_per_resolution = Money::from_major(18, iso::USD);
/// let spent = remediation_cost(31_500, cost_per_resolution).unwrap();
/// assert_eq!(spent, Money::from_major(567_000, iso::USD));
/// ```
pub fn remediation_cost(
    duplicates_resolved: u32,
    cost_per_resolution: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_per_resolution.mul(duplicates_resolved)
}

/// The backlog cost: duplicates not yet resolved multiplied by the cost of
/// one resolution.
///
/// # Arguments
///
/// * `duplicates_unresolved` — identified duplicates not yet reviewed and
///   merged (count).
/// * `cost_per_resolution` — health information management staff cost to
///   research, verify, and merge one duplicate record.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `cost_per_resolution`'s underlying decimal representation.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::duplicate_record_remediation_cost::backlog_cost;
///
/// // Worked example: 10,500 × $18.00 = $189,000.00.
/// let cost_per_resolution = Money::from_major(18, iso::USD);
/// let backlog = backlog_cost(10_500, cost_per_resolution).unwrap();
/// assert_eq!(backlog, Money::from_major(189_000, iso::USD));
/// ```
pub fn backlog_cost(
    duplicates_unresolved: u32,
    cost_per_resolution: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    cost_per_resolution.mul(duplicates_unresolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "31,500 × $18.00 = $567,000.00 already spent."
    #[test]
    fn worked_example_remediation_cost_is_567_000_usd() {
        let cost_per_resolution = Money::from_major(18, iso::USD);
        let spent = remediation_cost(31_500, cost_per_resolution).unwrap();
        assert_eq!(spent, Money::from_major(567_000, iso::USD));
    }

    // Doc line: "10,500 × $18.00 = $189,000.00 in outstanding backlog cost."
    #[test]
    fn worked_example_backlog_cost_is_189_000_usd() {
        let cost_per_resolution = Money::from_major(18, iso::USD);
        let backlog = backlog_cost(10_500, cost_per_resolution).unwrap();
        assert_eq!(backlog, Money::from_major(189_000, iso::USD));
    }

    #[test]
    fn remediation_and_backlog_together_cover_all_identified_duplicates() {
        let cost_per_resolution = Money::from_major(18, iso::USD);
        let spent = remediation_cost(31_500, cost_per_resolution).unwrap();
        let backlog = backlog_cost(10_500, cost_per_resolution).unwrap();
        let all_identified_at_rate = cost_per_resolution.mul(42_000_u32).unwrap();
        assert_eq!(spent.add(backlog).unwrap(), all_identified_at_rate);
    }
}

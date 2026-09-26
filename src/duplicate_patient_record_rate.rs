//! # Duplicate Patient Record Rate
//!
//! Duplicate patient record rate is the share of a master patient index
//! (or enterprise master patient index) that a matching audit identifies as
//! likely belonging to a patient who already has another record, and the
//! share of those identified duplicates that have actually been reviewed
//! and merged. A duplicate record splits one patient's history across two
//! charts, which can hide a known allergy, medication, or problem list at
//! the point of care — a direct patient-safety risk, not merely a
//! data-quality nuisance.
//!
//! ## How it's calculated
//!
//! ```text
//! Duplicate record rate = duplicate records identified / total records × 100
//!
//! Resolved-duplicate rate = duplicates merged / duplicates identified × 100
//! ```
//!
//! ## Why it matters
//!
//! A duplicate record is the silent failure mode behind interoperability:
//! two systems can each individually believe they are exchanging a
//! complete record while each is only sending half of a fragmented patient
//! history. Published patient-matching literature commonly cites duplicate
//! rates in individual EHR systems ranging from roughly 8% up to over 20%,
//! rising further after a merger, acquisition, or EHR migration — the most
//! common triggers for a sudden spike — so the rate needs periodic
//! remeasurement rather than a one-time check. Identifying a duplicate is
//! also not the same as fixing it: the resolved-duplicate rate is the
//! governance measure that says whether identified duplicates are actually
//! being merged, the same distinction [clinical alert override
//! rate](crate::clinical_alert_override_rate) draws between an override
//! occurring and an override being documented.
//!
//! ## Worked example
//!
//! A hospital's master patient index holds 500,000 unique patient records.
//! A matching audit identifies 42,000 as likely duplicates; of those,
//! 31,500 have been reviewed and merged by the health information
//! management team.
//!
//! ```rust
//! use digital_health::duplicate_patient_record_rate::{duplicate_record_rate, resolved_duplicate_rate};
//!
//! // 42,000 / 500,000 × 100 = 8.4%.
//! let duplicate_rate = duplicate_record_rate(42_000.0, 500_000.0).unwrap();
//! assert!((duplicate_rate - 8.4).abs() < 1e-9);
//!
//! // 31,500 / 42,000 × 100 = 75% of identified duplicates have been merged.
//! let resolved = resolved_duplicate_rate(31_500.0, 42_000.0).unwrap();
//! assert!((resolved - 75.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The master patient index (MPI) or enterprise master patient index
//! (EMPI)'s own matching and deduplication reports are the primary source
//! for identified duplicates; the health information management (HIM)
//! department's merge log is the primary source for resolutions.
//!
//! ## Pitfalls
//!
//! - **Treating "identified" as equivalent to "resolved"** — an identified
//!   duplicate that is never merged provides none of the safety benefit;
//!   track the resolved-duplicate rate separately rather than reporting
//!   identification alone.
//! - **Comparing raw rates across organizations with different matching
//!   algorithms** — a stricter probabilistic matching algorithm surfaces
//!   more candidate duplicates than a looser deterministic one, inflating
//!   the apparent rate independent of true underlying data quality.
//! - **Not remeasuring after a merger, acquisition, or EHR migration** —
//!   these events are the most common cause of a sudden spike in duplicate
//!   records, and a rate measured before one is stale immediately after it.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov and Pew Charitable Trusts research on patient
//!   identity matching and duplicate record rates.
//! - AHIMA (American Health Information Management Association), guidance
//!   on managing duplicate health records.
//! - Peer-reviewed literature on patient-matching algorithm accuracy,
//!   including studies published in JAMIA.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The duplicate record rate: records identified as likely duplicates, as a
/// percentage of total records in the index.
///
/// # Arguments
///
/// * `duplicate_records` — records a matching audit identifies as likely
///   duplicates (count).
/// * `total_records` — total unique patient records in the index (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_records` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::duplicate_patient_record_rate::duplicate_record_rate;
///
/// // Worked example: 42,000 / 500,000 × 100 = 8.4%.
/// let rate = duplicate_record_rate(42_000.0, 500_000.0).unwrap();
/// assert!((rate - 8.4).abs() < 1e-9);
///
/// assert!(duplicate_record_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn duplicate_record_rate(duplicate_records: f64, total_records: f64) -> Option<f64> {
    if total_records == 0.0 {
        None
    } else {
        Some(duplicate_records / total_records * 100.0)
    }
}

/// The resolved-duplicate rate: identified duplicates that have been
/// reviewed and merged, as a percentage of all identified duplicates.
///
/// A governance measure in its own right, separate from the duplicate
/// record rate itself: it says whether identified duplicates are actually
/// being fixed, not merely how many exist.
///
/// # Arguments
///
/// * `resolved_duplicates` — identified duplicates that have been reviewed
///   and merged (count).
/// * `identified_duplicates` — total duplicates identified by the matching
///   audit (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `identified_duplicates` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::duplicate_patient_record_rate::resolved_duplicate_rate;
///
/// // Worked example: 31,500 / 42,000 × 100 = 75%.
/// let rate = resolved_duplicate_rate(31_500.0, 42_000.0).unwrap();
/// assert!((rate - 75.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn resolved_duplicate_rate(
    resolved_duplicates: f64,
    identified_duplicates: f64,
) -> Option<f64> {
    if identified_duplicates == 0.0 {
        None
    } else {
        Some(resolved_duplicates / identified_duplicates * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "A matching audit identifies 42,000 as likely duplicates" —
    // 8.4% of the 500,000-record index.
    #[test]
    fn worked_example_duplicate_record_rate_is_8_4_percent() {
        let rate = duplicate_record_rate(42_000.0, 500_000.0).unwrap();
        assert!((rate - 8.4).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "31,500 have been reviewed and merged" — 75% of identified
    // duplicates.
    #[test]
    fn worked_example_resolved_duplicate_rate_is_75_percent() {
        let rate = resolved_duplicate_rate(31_500.0, 42_000.0).unwrap();
        assert!((rate - 75.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(duplicate_record_rate(1.0, 0.0).is_none());
        assert!(resolved_duplicate_rate(1.0, 0.0).is_none());
    }
}

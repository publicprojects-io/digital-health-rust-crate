//! # ED Diversion Rate
//!
//! Emergency department (ED) diversion rate is the share of virtual-triage
//! contacts that end in avoided ED visits — patients who would otherwise
//! have attended and were safely directed to a lower-acuity route — and the
//! safe diversion rate, the share of diverted patients who did not return
//! to an ED soon after.
//!
//! ## How it's calculated
//!
//! ```text
//! ED diversion rate = diverted contacts / triage contacts × 100
//!
//! Safe diversion rate = diverted contacts with no ED return within 72 hours / diverted contacts × 100
//! ```
//!
//! ## Why it matters
//!
//! ED diversion is the headline value claim for algorithmic virtual triage,
//! and the one most prone to overstatement: a diversion that leads to a
//! return ED visit or worse outcome is not a saving. Reporting the safe
//! diversion rate beside the raw rate keeps the safety signal in view, like
//! [`triage_accuracy_rate`](crate::triage_accuracy_rate)'s under-triage
//! measure.
//!
//! ## Worked example
//!
//! A virtual triage service handles 5,000 contacts. Reviewers judge 1,250
//! would otherwise have gone to an ED and were redirected. Of those, 1,150
//! had no ED visit within 72 hours.
//!
//! ```rust
//! use digital_health::ed_diversion_rate::{ed_diversion_rate, safe_diversion_rate};
//!
//! // 1,250 / 5,000 × 100 = 25% diverted.
//! let ed_diversion_rate_value = ed_diversion_rate(1_250.0, 5_000.0).unwrap();
//! assert!((ed_diversion_rate_value - 25.0).abs() < 1e-9);
//!
//! // 1,150 / 1,250 × 100 = 92% safe.
//! let safe_diversion_rate_value = safe_diversion_rate(1_150.0, 1_250.0).unwrap();
//! assert!((safe_diversion_rate_value - 92.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Whether a contact "would otherwise have gone to the ED" is a
//! counterfactual: use patient-reported intent captured at contact, or a
//! matched comparison, and state which. Follow-up needs linkage to ED
//! attendance records.
//!
//! ## Pitfalls
//!
//! - Counting every contact that wasn't sent to an ED as diverted.
//! - Ignoring return visits, which turn a diversion into a delay.
//! - Comparing services that define the counterfactual differently.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on telephone and digital triage and emergency
//!   department attendance.
//! - NHS 111 online and NHS England urgent and emergency care evaluations.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The ED diversion rate: contacts that avoided an ED visit as a percentage
/// of triage contacts.
///
/// # Arguments
///
/// * `diverted_contacts` — contacts judged to have avoided an ED visit
///   (count).
/// * `triage_contacts` — virtual triage contacts in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `triage_contacts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::ed_diversion_rate::ed_diversion_rate;
///
/// // Worked example: 1,250 / 5,000 × 100 = 25%.
/// let value = ed_diversion_rate(1_250.0, 5_000.0).unwrap();
/// assert!((value - 25.0).abs() < 1e-9);
///
/// assert!(ed_diversion_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn ed_diversion_rate(diverted_contacts: f64, triage_contacts: f64) -> Option<f64> {
    if triage_contacts == 0.0 {
        None
    } else {
        Some(diverted_contacts / triage_contacts * 100.0)
    }
}

/// The safe diversion rate: diverted contacts with no ED return in the
/// follow-up window, as a percentage of diverted contacts.
///
/// # Arguments
///
/// * `diverted_without_return` — diverted contacts with no ED visit in the
///   window (count).
/// * `diverted_contacts` — contacts judged to have avoided an ED visit
///   (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `diverted_contacts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::ed_diversion_rate::safe_diversion_rate;
///
/// // Worked example: 1,150 / 1,250 × 100 = 92%.
/// let value = safe_diversion_rate(1_150.0, 1_250.0).unwrap();
/// assert!((value - 92.0).abs() < 1e-9);
///
/// assert!(safe_diversion_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn safe_diversion_rate(diverted_without_return: f64, diverted_contacts: f64) -> Option<f64> {
    if diverted_contacts == 0.0 {
        None
    } else {
        Some(diverted_without_return / diverted_contacts * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "1,250 / 5,000 × 100 = 25% diverted."
    #[test]
    fn worked_example_ed_diversion_rate() {
        let value = ed_diversion_rate(1_250.0, 5_000.0).unwrap();
        assert!((value - 25.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "1,150 / 1,250 × 100 = 92% safe."
    #[test]
    fn worked_example_safe_diversion_rate() {
        let value = safe_diversion_rate(1_150.0, 1_250.0).unwrap();
        assert!((value - 92.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(ed_diversion_rate(1.0, 0.0).is_none());
        assert!(safe_diversion_rate(1.0, 0.0).is_none());
    }
}

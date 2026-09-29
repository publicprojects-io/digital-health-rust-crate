//! # Triage Accuracy Rate
//!
//! Triage accuracy rate is the share of digital triage decisions — from a
//! symptom checker, AI triage assistant, or nurse-line decision-support
//! tool — that match a clinician-reviewed reference standard for the care
//! level the patient actually needed. Under-triage rate is the share sent to
//! a *lower* care level than needed, the error that carries the clinical
//! safety risk.
//!
//! ## How it's calculated
//!
//! ```text
//! Triage accuracy rate = triage decisions matching reference / decisions reviewed × 100
//!
//! Under-triage rate = decisions routed below the needed care level / decisions reviewed × 100
//! ```
//!
//! ## Why it matters
//!
//! A single overall accuracy figure hides the asymmetry between the two
//! error types. Over-triage wastes capacity and patient time; under-triage
//! can delay care for someone who needed it urgently. Reporting
//! [`under_triage_rate`] beside [`triage_accuracy_rate`] keeps the safety
//! signal visible, and lets a tool be tuned deliberately toward the safer
//! error.
//!
//! ## Worked example
//!
//! A digital front door reviews 1,000 AI-assisted triage sessions against
//! a panel of nurse reviewers' agreed reference disposition. 870 match the
//! reference; 60 were routed to a lower care level than the reference
//! (under-triage) and the remaining 70 to a higher one (over-triage).
//!
//! ```rust
//! use digital_health::triage_accuracy_rate::{triage_accuracy_rate, under_triage_rate};
//!
//! // 870 / 1,000 × 100 = 87%.
//! let accuracy = triage_accuracy_rate(870.0, 1_000.0).unwrap();
//! assert!((accuracy - 87.0).abs() < 1e-9);
//!
//! // 60 / 1,000 × 100 = 6% under-triaged.
//! let under = under_triage_rate(60.0, 1_000.0).unwrap();
//! assert!((under - 6.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The tool's own disposition log supplies the decision; the reference
//! standard must come from an independent clinician review (or eventual
//! outcome), never from the tool's own output. Sample sessions randomly,
//! not only from complaints or escalations.
//!
//! ## Pitfalls
//!
//! - **Reporting accuracy alone** — a tool can score well overall while
//!   under-triaging exactly the high-acuity cases that matter most.
//! - **Reviewing a non-random sample** — reviewing only escalated or
//!   complained-about sessions inflates the apparent error rate.
//! - **Comparing across care-level schemes** — a 3-level and a 5-level
//!   acuity scale (such as the Emergency Severity Index) produce
//!   incomparable accuracy figures.
//!
//! ## Sources
//!
//! - Emergency Severity Index (ESI) implementation handbook, on under- and
//!   over-triage definitions.
//! - Peer-reviewed literature on symptom-checker and digital triage safety
//!   evaluation against clinician reference dispositions.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The triage accuracy rate: decisions matching the reference standard as a
/// percentage of decisions reviewed.
///
/// # Arguments
///
/// * `matching_reference` — reviewed triage decisions that matched the
///   reference care level (count).
/// * `reviewed` — total triage decisions reviewed in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `reviewed` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::triage_accuracy_rate::triage_accuracy_rate;
///
/// // Worked example: 870 / 1,000 × 100 = 87%.
/// let rate = triage_accuracy_rate(870.0, 1_000.0).unwrap();
/// assert!((rate - 87.0).abs() < 1e-9);
///
/// assert!(triage_accuracy_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn triage_accuracy_rate(matching_reference: f64, reviewed: f64) -> Option<f64> {
    if reviewed == 0.0 {
        None
    } else {
        Some(matching_reference / reviewed * 100.0)
    }
}

/// The under-triage rate: decisions routed below the needed care level as a
/// percentage of decisions reviewed.
///
/// # Arguments
///
/// * `under_triaged` — reviewed decisions routed to a lower care level than
///   the reference (count).
/// * `reviewed` — total triage decisions reviewed in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `reviewed` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::triage_accuracy_rate::under_triage_rate;
///
/// // Worked example: 60 / 1,000 × 100 = 6%.
/// let rate = under_triage_rate(60.0, 1_000.0).unwrap();
/// assert!((rate - 6.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn under_triage_rate(under_triaged: f64, reviewed: f64) -> Option<f64> {
    if reviewed == 0.0 {
        None
    } else {
        Some(under_triaged / reviewed * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "870 match the reference" out of 1,000 — 87%.
    #[test]
    fn worked_example_triage_accuracy_rate_is_87_percent() {
        let rate = triage_accuracy_rate(870.0, 1_000.0).unwrap();
        assert!((rate - 87.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "60 were routed to a lower care level" — 6%.
    #[test]
    fn worked_example_under_triage_rate_is_6_percent() {
        let rate = under_triage_rate(60.0, 1_000.0).unwrap();
        assert!((rate - 6.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(triage_accuracy_rate(1.0, 0.0).is_none());
        assert!(under_triage_rate(1.0, 0.0).is_none());
    }
}

//! # Engagement Consistency Rate
//!
//! Engagement consistency rate tracks how steadily patients keep using a
//! digital tool over time: the share of enrolled weeks in which a patient
//! was active (logging food, activity, or viewing their health stats), and
//! the dropout rate — the share of enrolled patients who stop engaging.
//!
//! ## How it's calculated
//!
//! ```text
//! Active week rate = active weeks / enrolled weeks × 100
//!
//! Dropout rate = patients who dropped out / patients enrolled × 100
//! ```
//!
//! ## Why it matters
//!
//! Sustained interaction is a strong predictor of clinical improvement, and
//! a single login count cannot show it. Active-week share captures
//! consistency, and dropout captures loss. It complements the fixed day-7
//! and day-30 checkpoints in [digital therapeutic retention
//! rate](crate::digital_therapeutic_retention_rate) with a continuous view.
//!
//! ## Worked example
//!
//! A weight-management cohort has 480 enrolled patient-weeks in a month, of
//! which patients were active in 312. Of 1,200 patients who enrolled this
//! quarter, 180 dropped out.
//!
//! ```rust
//! use digital_health::engagement_consistency_rate::{active_week_rate, dropout_rate};
//!
//! // 312 / 480 × 100 = 65% of weeks active.
//! let active_week_rate_value = active_week_rate(312.0, 480.0).unwrap();
//! assert!((active_week_rate_value - 65.0).abs() < 1e-9);
//!
//! // 180 / 1,200 × 100 = 15% dropped out.
//! let dropout_rate_value = dropout_rate(180.0, 1_200.0).unwrap();
//! assert!((dropout_rate_value - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Define "active" once as a meaningful action, not a bare login, and
//! define dropout as a fixed period of inactivity (for example 28 days).
//! Count a patient's enrolled weeks only from enrolment to disenrolment or
//! study end.
//!
//! ## Pitfalls
//!
//! - Counting a login as engagement — passive opens don't predict outcomes.
//! - Changing the inactivity window for dropout between periods.
//! - Reporting the rate for survivors only, which hides attrition.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on engagement and retention in digital health
//!   interventions.
//! - Eysenbach, G. "The Law of Attrition." *Journal of Medical Internet
//!   Research*, 2005.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The active week rate: enrolled weeks with meaningful activity as a
/// percentage of all enrolled weeks.
///
/// # Arguments
///
/// * `active_weeks` — patient-weeks with at least one meaningful action
///   (count).
/// * `enrolled_weeks` — total patient-weeks enrolled (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `enrolled_weeks` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::engagement_consistency_rate::active_week_rate;
///
/// // Worked example: 312 / 480 × 100 = 65%.
/// let value = active_week_rate(312.0, 480.0).unwrap();
/// assert!((value - 65.0).abs() < 1e-9);
///
/// assert!(active_week_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn active_week_rate(active_weeks: f64, enrolled_weeks: f64) -> Option<f64> {
    if enrolled_weeks == 0.0 {
        None
    } else {
        Some(active_weeks / enrolled_weeks * 100.0)
    }
}

/// The dropout rate: patients who stopped engaging as a percentage of
/// patients enrolled.
///
/// # Arguments
///
/// * `dropouts` — patients inactive beyond the dropout window (count).
/// * `enrolled` — patients enrolled (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `enrolled` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::engagement_consistency_rate::dropout_rate;
///
/// // Worked example: 180 / 1,200 × 100 = 15%.
/// let value = dropout_rate(180.0, 1_200.0).unwrap();
/// assert!((value - 15.0).abs() < 1e-9);
///
/// assert!(dropout_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn dropout_rate(dropouts: f64, enrolled: f64) -> Option<f64> {
    if enrolled == 0.0 {
        None
    } else {
        Some(dropouts / enrolled * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "312 / 480 × 100 = 65% of weeks active."
    #[test]
    fn worked_example_active_week_rate() {
        let value = active_week_rate(312.0, 480.0).unwrap();
        assert!((value - 65.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "180 / 1,200 × 100 = 15% dropped out."
    #[test]
    fn worked_example_dropout_rate() {
        let value = dropout_rate(180.0, 1_200.0).unwrap();
        assert!((value - 15.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(active_week_rate(1.0, 0.0).is_none());
        assert!(dropout_rate(1.0, 0.0).is_none());
    }
}

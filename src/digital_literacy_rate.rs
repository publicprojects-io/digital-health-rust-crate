//! # Digital Literacy Rate
//!
//! Digital literacy rate measures whether patients can actually use a
//! digital health service: the share of attempted telehealth or portal
//! tasks completed unassisted, and the share needing help. It is the second
//! equity dimension after access, stratified by language, age, and
//! socioeconomic group.
//!
//! ## How it's calculated
//!
//! ```text
//! Unassisted completion rate = tasks completed unassisted / tasks attempted × 100
//!
//! Assistance rate = tasks needing assistance / tasks attempted × 100
//! ```
//!
//! ## Why it matters
//!
//! Access to a device and broadband does not mean a patient can join a
//! video visit or use the portal. Behavioural measures like unassisted
//! completion avoid the bias of self-reported confidence, and computing
//! them per demographic stratum shows who the service is quietly failing.
//!
//! ## Worked example
//!
//! Of 800 first telehealth visit attempts in a quarter, 640 were completed
//! with no staff or family help and 120 needed phone or in-person
//! assistance.
//!
//! ```rust
//! use digital_health::digital_literacy_rate::{unassisted_completion_rate, assistance_rate};
//!
//! // 640 / 800 × 100 = 80% unassisted.
//! let unassisted_completion_rate_value = unassisted_completion_rate(640.0, 800.0).unwrap();
//! assert!((unassisted_completion_rate_value - 80.0).abs() < 1e-9);
//!
//! // 120 / 800 × 100 = 15% needed help.
//! let assistance_rate_value = assistance_rate(120.0, 800.0).unwrap();
//! assert!((assistance_rate_value - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Task logs from the platform and help-desk tickets are the behavioural
//! source; validated survey instruments such as the eHealth Literacy Scale
//! (eHEALS) complement them. Compute both rates for every language and age
//! stratum, not just overall.
//!
//! ## Pitfalls
//!
//! - Reporting only the overall rate — it hides the strata with the largest
//!   gaps.
//! - Counting abandoned attempts as neither completed nor assisted —
//!   attempts that silently fail are the strongest literacy signal.
//! - Assuming assistance always signals low literacy; a bad interface
//!   causes it too.
//!
//! ## Sources
//!
//! - HIMSS Digital Health Equity Measurement Framework (DHEMF).
//! - Norman, C. D. and Skinner, H. A. "eHEALS: The eHealth Literacy Scale."
//!   *Journal of Medical Internet Research*, 2006.
//! - Peer-reviewed literature on digital literacy barriers to telehealth
//!   use.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The unassisted completion rate: tasks completed without help as a
/// percentage of tasks attempted.
///
/// # Arguments
///
/// * `completed_unassisted` — tasks completed with no assistance (count).
/// * `attempted` — tasks attempted (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `attempted` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_literacy_rate::unassisted_completion_rate;
///
/// // Worked example: 640 / 800 × 100 = 80%.
/// let value = unassisted_completion_rate(640.0, 800.0).unwrap();
/// assert!((value - 80.0).abs() < 1e-9);
///
/// assert!(unassisted_completion_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn unassisted_completion_rate(completed_unassisted: f64, attempted: f64) -> Option<f64> {
    if attempted == 0.0 {
        None
    } else {
        Some(completed_unassisted / attempted * 100.0)
    }
}

/// The assistance rate: tasks needing staff or family help as a percentage
/// of tasks attempted.
///
/// # Arguments
///
/// * `needed_assistance` — tasks that needed assistance (count).
/// * `attempted` — tasks attempted (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `attempted` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::digital_literacy_rate::assistance_rate;
///
/// // Worked example: 120 / 800 × 100 = 15%.
/// let value = assistance_rate(120.0, 800.0).unwrap();
/// assert!((value - 15.0).abs() < 1e-9);
///
/// assert!(assistance_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn assistance_rate(needed_assistance: f64, attempted: f64) -> Option<f64> {
    if attempted == 0.0 {
        None
    } else {
        Some(needed_assistance / attempted * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "640 / 800 × 100 = 80% unassisted."
    #[test]
    fn worked_example_unassisted_completion_rate() {
        let value = unassisted_completion_rate(640.0, 800.0).unwrap();
        assert!((value - 80.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "120 / 800 × 100 = 15% needed help."
    #[test]
    fn worked_example_assistance_rate() {
        let value = assistance_rate(120.0, 800.0).unwrap();
        assert!((value - 15.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(unassisted_completion_rate(1.0, 0.0).is_none());
        assert!(assistance_rate(1.0, 0.0).is_none());
    }
}

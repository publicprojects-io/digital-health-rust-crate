//! # Patient Identity Verification Rate
//!
//! Patient identity verification rate is the share of patient portal
//! identity-verification attempts that succeed, and the share of attempts
//! that require a stronger, step-up identity-proofing step (such as
//! document-based verification) before succeeding or failing. It is the
//! gate at the very start of [patient portal adoption
//! rate](crate::patient_portal_adoption_rate)'s own funnel: a patient who
//! fails verification never reaches registration at all.
//!
//! ## How it's calculated
//!
//! ```text
//! Verification success rate = verified identities / verification attempts × 100
//!
//! Step-up verification rate = attempts requiring step-up verification /
//!                              verification attempts × 100
//! ```
//!
//! ## Why it matters
//!
//! A low verification success rate can look like a portal-adoption problem
//! when it is actually an identity-proofing problem further upstream, with
//! a different fix (identity-proofing vendor tuning, not portal marketing).
//! Step-up verification rate matters separately because each step-up event
//! is exactly the kind of friction correlated with the same
//! digital-exclusion factors — lack of a smartphone camera, unfamiliarity
//! with the process — that [patient portal adoption
//! rate](crate::patient_portal_adoption_rate) warns can widen access gaps.
//!
//! ## Worked example
//!
//! A health system's patient portal processes 5,000 new-account identity
//! verification attempts in a month; 4,250 succeed. Of all attempts, 750
//! require a step-up, document-based verification step.
//!
//! ```rust
//! use digital_health::patient_identity_verification_rate::{verification_success_rate, step_up_verification_rate};
//!
//! // 4,250 / 5,000 × 100 = 85%.
//! let success = verification_success_rate(4_250.0, 5_000.0).unwrap();
//! assert!((success - 85.0).abs() < 1e-9);
//!
//! // 750 / 5,000 × 100 = 15%.
//! let step_up = step_up_verification_rate(750.0, 5_000.0).unwrap();
//! assert!((step_up - 15.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The identity-proofing vendor's or portal platform's own verification log
//! is the primary source, using its outcome (verified, failed) and
//! assurance-path (base level, step-up) fields.
//!
//! ## Pitfalls
//!
//! - **Treating verification failure as identical to portal-adoption
//!   failure** — it is an earlier funnel stage with a different remediation.
//! - **Not tracking step-up rate separately** — a blended success rate hides
//!   how much friction the security model is actually imposing on
//!   legitimate patients.
//! - **Comparing success rates across vendors without controlling for
//!   assurance level** — a stricter standard (e.g. NIST IAL2 versus IAL1)
//!   shows a lower success rate by design, not by worse execution.
//!
//! ## Sources
//!
//! - NIST Special Publication 800-63, Digital Identity Guidelines.
//! - ONC / HealthIT.gov patient identity and portal access guidance.
//! - Peer-reviewed literature on patient portal registration barriers.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The verification success rate: identity verification attempts that
/// succeed, as a percentage of total attempts.
///
/// # Arguments
///
/// * `verified` — verification attempts that succeed (count).
/// * `attempts` — total verification attempts in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `attempts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_identity_verification_rate::verification_success_rate;
///
/// // Worked example: 4,250 / 5,000 × 100 = 85%.
/// let rate = verification_success_rate(4_250.0, 5_000.0).unwrap();
/// assert!((rate - 85.0).abs() < 1e-9);
///
/// assert!(verification_success_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn verification_success_rate(verified: f64, attempts: f64) -> Option<f64> {
    if attempts == 0.0 {
        None
    } else {
        Some(verified / attempts * 100.0)
    }
}

/// The step-up verification rate: attempts requiring a stronger,
/// document-based verification step, as a percentage of total attempts.
///
/// # Arguments
///
/// * `step_up_required` — verification attempts requiring a step-up
///   verification step (count).
/// * `attempts` — total verification attempts in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `attempts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_identity_verification_rate::step_up_verification_rate;
///
/// // Worked example: 750 / 5,000 × 100 = 15%.
/// let rate = step_up_verification_rate(750.0, 5_000.0).unwrap();
/// assert!((rate - 15.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn step_up_verification_rate(step_up_required: f64, attempts: f64) -> Option<f64> {
    if attempts == 0.0 {
        None
    } else {
        Some(step_up_required / attempts * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "4,250 succeed" out of 5,000 attempts — 85%.
    #[test]
    fn worked_example_verification_success_rate_is_85_percent() {
        let rate = verification_success_rate(4_250.0, 5_000.0).unwrap();
        assert!((rate - 85.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "750 require a step-up, document-based verification step" —
    // 15%.
    #[test]
    fn worked_example_step_up_rate_is_15_percent() {
        let rate = step_up_verification_rate(750.0, 5_000.0).unwrap();
        assert!((rate - 15.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(verification_success_rate(1.0, 0.0).is_none());
        assert!(step_up_verification_rate(1.0, 0.0).is_none());
    }
}

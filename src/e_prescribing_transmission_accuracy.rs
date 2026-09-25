//! # E-Prescribing Transmission Accuracy
//!
//! E-prescribing transmission accuracy measures the share of electronic
//! prescriptions that reach the pharmacy in a state the pharmacist can fill
//! without contacting the prescriber for clarification, versus the share
//! that generate a pharmacy call-back. Electronic transmission removes the
//! classic handwriting-legibility error, but it introduces a different
//! failure mode — a structured-data mapping problem (an unmapped sig, the
//! wrong strength code, a missing quantity) that a network delivery receipt
//! alone cannot detect, because the message can transmit cleanly and still
//! be wrong or unfillable as written.
//!
//! ## How it's calculated
//!
//! ```text
//! First-pass transmission rate = transmissions with no pharmacy call-back /
//!                                 total transmissions × 100
//!
//! Pharmacy call-back rate = transmissions with a pharmacy call-back /
//!                            total transmissions × 100
//! ```
//!
//! Segment call-backs by reason where the prescribing or pharmacy system
//! captures one: a safety-relevant call-back (dose, strength, drug
//! interaction) and a purely administrative one (prior authorization,
//! formulary) need different fixes.
//!
//! ## Why it matters
//!
//! A "successfully transmitted" e-prescription is a network delivery
//! outcome, not a clinical-correctness outcome, so the call-back rate is
//! the standard operational proxy for the class of medication-safety risk
//! that transmission-success metrics alone miss. A rising rate driven by
//! data-mapping problems (wrong strength codes, unmapped free-text sigs) is
//! a prescribing-system quality issue; a rising rate driven by formulary or
//! prior-authorization call-backs is a payer/administrative issue — the two
//! require different responses, which is why segmenting by reason matters
//! more than the headline number.
//!
//! ## Worked example
//!
//! An outpatient clinic transmits 5,000 e-prescriptions electronically in a
//! month. Of these, 150 generate a pharmacy call-back — for a clarification,
//! a formulary or prior-authorization issue, or a data-mapping problem —
//! leaving 4,850 that fill without any clarification contact.
//!
//! ```rust
//! use digital_health::e_prescribing_transmission_accuracy::{
//!     first_pass_transmission_rate, pharmacy_callback_rate,
//! };
//!
//! // 4,850 / 5,000 × 100 = 97%.
//! let first_pass = first_pass_transmission_rate(4_850.0, 5_000.0).unwrap();
//! assert!((first_pass - 97.0).abs() < 1e-9);
//!
//! // 150 / 5,000 × 100 = 3%.
//! let callback = pharmacy_callback_rate(150.0, 5_000.0).unwrap();
//! assert!((callback - 3.0).abs() < 1e-9);
//!
//! // The two rates are complementary only when every transmission falls
//! // into exactly one of these two buckets, as in this worked example.
//! assert!((first_pass + callback - 100.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Pharmacy network transmission reports (e.g. from a prescription routing
//! network) and the prescribing system's own outbound transmission audit
//! log are the primary sources. Where captured, pharmacy call-back reason
//! codes let an organization distinguish safety-relevant call-backs from
//! administrative ones; without reason codes, only the blended rate is
//! available.
//!
//! ## Pitfalls
//!
//! - **Treating a clean network transmission as clinically correct** — a
//!   message can be delivered successfully and still contain an error a
//!   pharmacist has to catch.
//! - **Not segmenting call-back reason** — a rising rate driven entirely by
//!   formulary or prior-authorization issues is not a prescribing-system
//!   safety problem, and treating it as one misdirects the fix.
//! - **Comparing rates across systems or specialties without adjusting for
//!   medication complexity** — compounded medications, controlled
//!   substances, and high-alert medications generate more call-backs
//!   regardless of system quality.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov, e-prescribing and health IT safety guidance.
//! - Institute for Safe Medication Practices (ISMP), guidance on
//!   e-prescribing-related medication errors.
//! - Peer-reviewed literature and prescription-routing-network publications
//!   on e-prescription transmission quality and pharmacist call-back burden.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written; see Sources above.

/// The first-pass transmission rate: e-prescriptions transmitted without a
/// pharmacy call-back, as a percentage of all transmissions.
///
/// # Arguments
///
/// * `clean_transmissions` — transmissions the pharmacy filled without
///   contacting the prescriber (count).
/// * `total_transmissions` — total e-prescriptions transmitted in the
///   period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_transmissions` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::e_prescribing_transmission_accuracy::first_pass_transmission_rate;
///
/// // Worked example: 4,850 / 5,000 × 100 = 97%.
/// let rate = first_pass_transmission_rate(4_850.0, 5_000.0).unwrap();
/// assert!((rate - 97.0).abs() < 1e-9);
///
/// assert!(first_pass_transmission_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn first_pass_transmission_rate(
    clean_transmissions: f64,
    total_transmissions: f64,
) -> Option<f64> {
    if total_transmissions == 0.0 {
        None
    } else {
        Some(clean_transmissions / total_transmissions * 100.0)
    }
}

/// The pharmacy call-back rate: e-prescriptions that generated a pharmacy
/// call-back, as a percentage of all transmissions.
///
/// # Arguments
///
/// * `callbacks` — transmissions that generated a pharmacy call-back for
///   clarification (count).
/// * `total_transmissions` — total e-prescriptions transmitted in the
///   period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_transmissions` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::e_prescribing_transmission_accuracy::pharmacy_callback_rate;
///
/// // Worked example: 150 / 5,000 × 100 = 3%.
/// let rate = pharmacy_callback_rate(150.0, 5_000.0).unwrap();
/// assert!((rate - 3.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn pharmacy_callback_rate(callbacks: f64, total_transmissions: f64) -> Option<f64> {
    if total_transmissions == 0.0 {
        None
    } else {
        Some(callbacks / total_transmissions * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "leaving 4,850 that fill without any clarification contact"
    // — a first-pass transmission rate of 97%.
    #[test]
    fn worked_example_first_pass_rate_is_97_percent() {
        let rate = first_pass_transmission_rate(4_850.0, 5_000.0).unwrap();
        assert!((rate - 97.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "150 generate a pharmacy call-back" — a call-back rate of 3%.
    #[test]
    fn worked_example_callback_rate_is_3_percent() {
        let rate = pharmacy_callback_rate(150.0, 5_000.0).unwrap();
        assert!((rate - 3.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn rates_are_complementary_when_every_transmission_is_classified() {
        let first_pass = first_pass_transmission_rate(4_850.0, 5_000.0).unwrap();
        let callback = pharmacy_callback_rate(150.0, 5_000.0).unwrap();
        assert!((first_pass + callback - 100.0).abs() < 1e-9);
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(first_pass_transmission_rate(1.0, 0.0).is_none());
        assert!(pharmacy_callback_rate(1.0, 0.0).is_none());
    }
}

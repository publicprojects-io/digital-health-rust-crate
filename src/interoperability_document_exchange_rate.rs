//! # Interoperability Document Exchange Rate
//!
//! Interoperability document exchange rate is the share of clinical
//! document exchange transactions — commonly C-CDA summaries exchanged
//! through a health information exchange — that transmit successfully, and
//! the share of those successful transmissions that parse into structured,
//! discrete data at the receiving system rather than arriving as an opaque
//! attachment. A "successful" exchange is a transport-layer success, not a
//! usability success, and the gap between the two rates is the practical
//! measure of whether interoperability is actually reducing manual clinical
//! work.
//!
//! ## How it's calculated
//!
//! ```text
//! Exchange success rate = successful transmissions / total exchange attempts × 100
//!
//! Structured data parse rate = successfully parsed documents / successful transmissions × 100
//! ```
//!
//! ## Why it matters
//!
//! A C-CDA document that fails to parse still gets filed in the receiving
//! electronic health record — but as a document a clinician must open and
//! read manually, rather than data that populates the problem list,
//! medication list, or allergy list automatically. Reporting only the
//! exchange success rate and inferring usability from it overstates what
//! interoperability is actually delivering; the parse rate is the number
//! that says whether the data becomes usable, not just whether it arrived.
//!
//! ## Worked example
//!
//! A health information exchange processes 50,000 document exchange
//! transactions in a month; 47,500 transmit successfully. Of those, 38,000
//! parse into structured, discrete data fields at the receiving system.
//!
//! ```rust
//! use digital_health::interoperability_document_exchange_rate::{exchange_success_rate, structured_data_parse_rate};
//!
//! // 47,500 / 50,000 × 100 = 95%.
//! let success = exchange_success_rate(47_500.0, 50_000.0).unwrap();
//! assert!((success - 95.0).abs() < 1e-9);
//!
//! // 38,000 / 47,500 × 100 = 80% parse into usable structured data.
//! let parsed = structured_data_parse_rate(38_000.0, 47_500.0).unwrap();
//! assert!((parsed - 80.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The health information exchange's or interface engine's own transaction
//! log is the primary source for transmission success; the receiving EHR's
//! document-processing log, which records whether an incoming document was
//! reconciled into discrete data or filed as an attachment, is the primary
//! source for the parse rate.
//!
//! ## Pitfalls
//!
//! - **Reporting only transmission success and inferring usability** — the
//!   two rates measure different things and can diverge substantially.
//! - **Not accounting for template or version mismatches** — a common
//!   real-world cause of parse failure is a sending system using a C-CDA
//!   template variant the receiving system doesn't fully support.
//! - **Treating a parse failure as always a sender-side problem** — it is
//!   often a receiving-system configuration or mapping issue instead.
//!
//! ## Sources
//!
//! - ONC / HealthIT.gov interoperability standards and Trusted Exchange
//!   Framework and Common Agreement (TEFCA) guidance.
//! - Peer-reviewed literature on C-CDA data quality and usability in health
//!   information exchange.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The exchange success rate: document exchange transactions that transmit
/// successfully, as a percentage of total exchange attempts.
///
/// # Arguments
///
/// * `successful_transmissions` — document exchange transactions that
///   transmitted successfully (count).
/// * `total_exchange_attempts` — total document exchange transactions
///   attempted in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_exchange_attempts` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::interoperability_document_exchange_rate::exchange_success_rate;
///
/// // Worked example: 47,500 / 50,000 × 100 = 95%.
/// let rate = exchange_success_rate(47_500.0, 50_000.0).unwrap();
/// assert!((rate - 95.0).abs() < 1e-9);
///
/// assert!(exchange_success_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn exchange_success_rate(
    successful_transmissions: f64,
    total_exchange_attempts: f64,
) -> Option<f64> {
    if total_exchange_attempts == 0.0 {
        None
    } else {
        Some(successful_transmissions / total_exchange_attempts * 100.0)
    }
}

/// The structured data parse rate: successfully transmitted documents that
/// parse into structured, discrete data, as a percentage of all
/// successfully transmitted documents.
///
/// # Arguments
///
/// * `successfully_parsed` — successfully transmitted documents that parse
///   into structured, discrete data at the receiving system (count).
/// * `successful_transmissions` — total successfully transmitted documents
///   in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `successful_transmissions` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::interoperability_document_exchange_rate::structured_data_parse_rate;
///
/// // Worked example: 38,000 / 47,500 × 100 = 80%.
/// let rate = structured_data_parse_rate(38_000.0, 47_500.0).unwrap();
/// assert!((rate - 80.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn structured_data_parse_rate(
    successfully_parsed: f64,
    successful_transmissions: f64,
) -> Option<f64> {
    if successful_transmissions == 0.0 {
        None
    } else {
        Some(successfully_parsed / successful_transmissions * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "47,500 transmit successfully" out of 50,000 — 95%.
    #[test]
    fn worked_example_exchange_success_rate_is_95_percent() {
        let rate = exchange_success_rate(47_500.0, 50_000.0).unwrap();
        assert!((rate - 95.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "38,000 parse into structured, discrete data fields" — 80%.
    #[test]
    fn worked_example_parse_rate_is_80_percent() {
        let rate = structured_data_parse_rate(38_000.0, 47_500.0).unwrap();
        assert!((rate - 80.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(exchange_success_rate(1.0, 0.0).is_none());
        assert!(structured_data_parse_rate(1.0, 0.0).is_none());
    }
}

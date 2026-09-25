//! # Telehealth Visit Rate
//!
//! Telehealth visit rate is the share of a service's total encounters that
//! are delivered remotely, by video or telephone, rather than in person. It
//! is a mix-of-delivery-channel metric, not an activity metric: it tells you
//! how care is being delivered, which matters for capacity planning, access,
//! and clinical appropriateness, quite separately from how much care is
//! being delivered in total.
//!
//! ## How it's calculated
//!
//! ```text
//! Telehealth visit rate = telehealth encounters / (telehealth encounters + in-person encounters) × 100
//!
//! Report separately by modality where possible:
//!   Video rate     = video encounters / total encounters × 100
//!   Telephone rate = telephone-only encounters / total encounters × 100
//! ```
//!
//! Denominator should count completed encounters only, for a defined
//! service, specialty, and time period.
//!
//! ## Why it matters
//!
//! Telehealth is not a uniform substitute for an in-person visit:
//! appropriateness varies by specialty, by the type of consultation, and by
//! patient preference, so the "right" rate is a clinical and operational
//! judgement, not a target to maximize. Funders and regulators also use this
//! rate, alongside outcome and safety measures, to decide reimbursement
//! policy and to check that remote care is not simply being substituted onto
//! cases that need to be seen in person.
//!
//! ## Worked example
//!
//! A community mental health service records 4,000 completed outpatient
//! contacts in a quarter: 1,200 in person, 1,600 by video, and 1,200 by
//! telephone.
//!
//! ```rust
//! use digital_health::telehealth_visit_rate::{telehealth_visit_rate, video_rate, telephone_rate};
//!
//! // (1,600 + 1,200) / 4,000 × 100 = 70%.
//! let telehealth = telehealth_visit_rate(1_600.0 + 1_200.0, 1_200.0).unwrap();
//! assert!((telehealth - 70.0).abs() < 1e-9);
//!
//! // Video rate: 1,600 / 4,000 × 100 = 40%.
//! let video = video_rate(1_600.0, 4_000.0).unwrap();
//! assert!((video - 40.0).abs() < 1e-9);
//!
//! // Telephone-only rate: 1,200 / 4,000 × 100 = 30%.
//! let telephone = telephone_rate(1_200.0, 4_000.0).unwrap();
//! assert!((telephone - 30.0).abs() < 1e-9);
//!
//! // Reporting the combined 70% alone would obscure that a large share is audio-only.
//! assert!((video + telephone - telehealth).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Encounter type is usually recorded either as a structured field in the
//! electronic health record (visit type or location) or inferred from
//! billing codes, such as a place-of-service code or a telehealth modifier
//! on a claim. Coding practice varies significantly between organizations
//! and even between clinicians in the same organization, so a rate
//! comparison across sites should first confirm that "telehealth" is being
//! coded the same way in each.
//!
//! ## Pitfalls
//!
//! - **Counting attempted rather than completed visits** — a telehealth
//!   appointment that fails to connect and is rebooked should not inflate
//!   the telehealth denominator twice.
//! - **Treating video and telephone as interchangeable** — they have
//!   different clinical and equity implications; always report them
//!   separately when possible.
//! - **Ignoring the no-show relationship** — no-show behaviour often differs
//!   by modality; see [appointment no-show rate](crate::appointment_no_show_rate)
//!   before drawing conclusions about "improved access" from a rising
//!   telehealth rate alone.
//! - **Treating a high rate as inherently good** — for some conditions and
//!   consultation types, an appropriate telehealth rate is low by clinical
//!   design, not by digital maturity failure.
//!
//! ## Sources
//!
//! - Centers for Medicare & Medicaid Services (CMS), Medicare telehealth
//!   utilization data and policy publications.
//! - NHS England, outpatient and community services activity statistics,
//!   including virtual/remote attendance breakdowns.
//! - Peer-reviewed literature on telehealth utilization trends and
//!   modality-specific outcomes.
//!
//! Topic doc: digital-health-metrics/locales/en-gb-oxendict/topics/telehealth-visit-rate/index.md

/// The telehealth visit rate: telehealth encounters as a percentage of all
/// completed encounters (telehealth plus in-person).
///
/// # Arguments
///
/// * `telehealth_encounters` — completed video and telephone encounters
///   combined (count).
/// * `in_person_encounters` — completed in-person encounters (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if both counts are zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::telehealth_visit_rate::telehealth_visit_rate;
///
/// // Worked example: (1,600 + 1,200) / 4,000 × 100 = 70%.
/// let rate = telehealth_visit_rate(2_800.0, 1_200.0).unwrap();
/// assert!((rate - 70.0).abs() < 1e-9);
///
/// assert!(telehealth_visit_rate(0.0, 0.0).is_none());
/// ```
pub fn telehealth_visit_rate(telehealth_encounters: f64, in_person_encounters: f64) -> Option<f64> {
    let total = telehealth_encounters + in_person_encounters;
    if total == 0.0 {
        None
    } else {
        Some(telehealth_encounters / total * 100.0)
    }
}

/// The video rate: video encounters as a percentage of total completed
/// encounters.
///
/// # Arguments
///
/// * `video_encounters` — completed video encounters (count).
/// * `total_encounters` — total completed encounters in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_encounters` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::telehealth_visit_rate::video_rate;
///
/// // Worked example: 1,600 / 4,000 × 100 = 40%.
/// let rate = video_rate(1_600.0, 4_000.0).unwrap();
/// assert!((rate - 40.0).abs() < 1e-9);
/// ```
pub fn video_rate(video_encounters: f64, total_encounters: f64) -> Option<f64> {
    if total_encounters == 0.0 {
        None
    } else {
        Some(video_encounters / total_encounters * 100.0)
    }
}

/// The telephone-only rate: telephone-only encounters as a percentage of
/// total completed encounters.
///
/// # Arguments
///
/// * `telephone_encounters` — completed telephone-only encounters (count).
/// * `total_encounters` — total completed encounters in the period (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `total_encounters` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::telehealth_visit_rate::telephone_rate;
///
/// // Worked example: 1,200 / 4,000 × 100 = 30%.
/// let rate = telephone_rate(1_200.0, 4_000.0).unwrap();
/// assert!((rate - 30.0).abs() < 1e-9);
/// ```
pub fn telephone_rate(telephone_encounters: f64, total_encounters: f64) -> Option<f64> {
    if total_encounters == 0.0 {
        None
    } else {
        Some(telephone_encounters / total_encounters * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "The telehealth visit rate is (1,600 + 1,200) / 4,000 × 100 = 70%".
    #[test]
    fn worked_example_telehealth_rate_is_70_percent() {
        let rate = telehealth_visit_rate(2_800.0, 1_200.0).unwrap();
        assert!((rate - 70.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "a video rate of 40% and a telephone-only rate of 30%".
    #[test]
    fn worked_example_video_and_telephone_rates_match_doc() {
        let video = video_rate(1_600.0, 4_000.0).unwrap();
        let telephone = telephone_rate(1_200.0, 4_000.0).unwrap();
        assert!((video - 40.0).abs() < 1e-9, "got video {video}");
        assert!((telephone - 30.0).abs() < 1e-9, "got telephone {telephone}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(telehealth_visit_rate(0.0, 0.0).is_none());
        assert!(video_rate(1.0, 0.0).is_none());
        assert!(telephone_rate(1.0, 0.0).is_none());
    }
}

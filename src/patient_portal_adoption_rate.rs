//! # Patient Portal Adoption Rate
//!
//! Patient portal adoption rate measures the share of eligible patients who
//! have registered for, and actively use, an online patient portal (for
//! example NHS App, Patient Access, or an EHR-tethered portal such as
//! `MyChart`) to view records, book appointments, or message their care team.
//! It is the entry-level indicator of digital engagement: a patient who has
//! never activated an account cannot benefit from any downstream digital
//! service built on the portal.
//!
//! ## How it's calculated
//!
//! Report all three stages, not just registration, and always state the
//! denominator explicitly:
//!
//! ```text
//! Registration rate = patients with a portal account created / eligible patient population × 100
//! Activation rate   = patients who completed a first meaningful action (viewed a result,
//!                      booked a slot, sent a message) / patients with an account × 100
//! Active-use rate   = patients who logged in at least once in the trailing 12 months /
//!                      eligible patient population × 100
//! ```
//!
//! ## Why it matters
//!
//! A portal only creates value once a patient uses it, so organizations
//! should track adoption as a funnel rather than a single number:
//! registration, activation, and active use are three different rates that
//! get conflated far too often. Low or unevenly distributed adoption is also
//! an equity signal: patients who are older, have lower digital literacy, do
//! not speak the majority language, or lack reliable broadband or a
//! smartphone are systematically less likely to be counted in the
//! numerator, so a rising average adoption rate can mask a widening gap for
//! the patients who often need contact with services the most.
//!
//! ## Worked example
//!
//! A primary care network serves 50,000 eligible patients. Of these, 32,000
//! have registered for the portal. Of the 32,000 registrations, 27,000 have
//! completed at least one meaningful action. Over the last 12 months, 21,000
//! of the original 50,000 eligible patients logged in at least once.
//!
//! ```rust
//! use digital_health::patient_portal_adoption_rate::{
//!     registration_rate, activation_rate, active_use_rate,
//! };
//!
//! // 32,000 / 50,000 × 100 = 64%.
//! let registration = registration_rate(32_000.0, 50_000.0).unwrap();
//! assert!((registration - 64.0).abs() < 1e-9);
//!
//! // 27,000 / 32,000 × 100 = 84.375% (the doc rounds to 84%).
//! let activation = activation_rate(27_000.0, 32_000.0).unwrap();
//! assert!((activation - 84.375).abs() < 1e-9);
//!
//! // 21,000 / 50,000 × 100 = 42% — the number that should drive resourcing decisions.
//! let active_use = active_use_rate(21_000.0, 50_000.0).unwrap();
//! assert!((active_use - 42.0).abs() < 1e-9);
//!
//! // Reporting registration alone would considerably overstate real engagement.
//! assert!(active_use < registration);
//! ```
//!
//! ## Data sources and caveats
//!
//! Portal analytics typically come from the vendor platform itself (login
//! events, feature usage) or from the underlying electronic health record's
//! audit log. Denominator choice matters enormously: counting against the
//! total registered patient list rather than a genuinely eligible,
//! contactable population will always understate adoption, while counting
//! against only patients who were actively invited will always overstate
//! it, so the eligibility definition should be fixed and published alongside
//! every reported rate.
//!
//! ## Pitfalls
//!
//! - **Registration counted as adoption** — a created-but-never-used account
//!   has close to zero value; report activation and active use alongside
//!   registration, not instead of them.
//! - **Ignoring digital exclusion** — aggregate adoption figures can rise
//!   while the gap between the most- and least-digitally-included groups
//!   widens; always segment by age, deprivation, language, and disability
//!   where data governance allows it.
//! - **Comparing organizations with different eligibility definitions** —
//!   with no real difference in performance, definitions alone can move the
//!   reported rate.
//! - **Treating a one-off login as ongoing engagement** — a 12-month
//!   look-back is common, but a shorter window gives an earlier warning of
//!   declining use.
//!
//! ## Sources
//!
//! - NHS England, NHS App usage and registration statistics (nhs.uk /
//!   digital.nhs.uk publications).
//! - ONC / HealthIT.gov, Promoting Interoperability Program measures,
//!   including View, Download, Transmit (VDT) patient access measures.
//! - Peer-reviewed literature on patient portal adoption and digital health
//!   disparities, for example studies published in the Journal of the
//!   American Medical Informatics Association (JAMIA).
//!
//! Topic doc: digital-health-metrics/locales/en-gb-oxendict/topics/patient-portal-adoption-rate/index.md

/// The registration rate: patients with a portal account as a percentage of
/// the eligible patient population.
///
/// # Arguments
///
/// * `registered` — patients with a portal account created (count).
/// * `eligible` — eligible patient population (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `eligible` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_portal_adoption_rate::registration_rate;
///
/// // Worked example: 32,000 / 50,000 × 100 = 64%.
/// let rate = registration_rate(32_000.0, 50_000.0).unwrap();
/// assert!((rate - 64.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn registration_rate(registered: f64, eligible: f64) -> Option<f64> {
    if eligible == 0.0 {
        None
    } else {
        Some(registered / eligible * 100.0)
    }
}

/// The activation rate: registered patients who completed a first meaningful
/// action, as a percentage of registered patients.
///
/// # Arguments
///
/// * `activated` — registered patients who completed a first meaningful
///   action, such as viewing a result, booking a slot, or sending a message
///   (count).
/// * `registered` — patients with a portal account (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `registered` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_portal_adoption_rate::activation_rate;
///
/// // Worked example: 27,000 / 32,000 × 100 = 84.375%.
/// let rate = activation_rate(27_000.0, 32_000.0).unwrap();
/// assert!((rate - 84.375).abs() < 1e-9);
/// ```
#[must_use]
pub fn activation_rate(activated: f64, registered: f64) -> Option<f64> {
    if registered == 0.0 {
        None
    } else {
        Some(activated / registered * 100.0)
    }
}

/// The active-use rate: patients who logged in at least once in the trailing
/// window, as a percentage of the eligible patient population.
///
/// This is the funnel's harder, more honest number — the doc's worked
/// example warns that reporting registration alone considerably overstates
/// real engagement.
///
/// # Arguments
///
/// * `active_in_window` — eligible patients who logged in at least once in
///   the trailing window, commonly 12 months (count).
/// * `eligible` — eligible patient population (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `eligible` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::patient_portal_adoption_rate::active_use_rate;
///
/// // Worked example: 21,000 / 50,000 × 100 = 42%.
/// let rate = active_use_rate(21_000.0, 50_000.0).unwrap();
/// assert!((rate - 42.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn active_use_rate(active_in_window: f64, eligible: f64) -> Option<f64> {
    if eligible == 0.0 {
        None
    } else {
        Some(active_in_window / eligible * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "32,000 have registered for the portal (registration rate 64%)".
    #[test]
    fn worked_example_registration_rate_is_64_percent() {
        let rate = registration_rate(32_000.0, 50_000.0).unwrap();
        assert!((rate - 64.0).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "activation rate 84% of registrants" (exact value 84.375%).
    #[test]
    fn worked_example_activation_rate_is_84_375_percent() {
        let rate = activation_rate(27_000.0, 32_000.0).unwrap();
        assert!((rate - 84.375).abs() < 1e-9, "got {rate}");
    }

    // Doc line: "active-use rate 42% ... is the number that should drive
    // resourcing decisions for the portal programme".
    #[test]
    fn worked_example_active_use_rate_is_42_percent() {
        let rate = active_use_rate(21_000.0, 50_000.0).unwrap();
        assert!((rate - 42.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(registration_rate(1.0, 0.0).is_none());
        assert!(activation_rate(1.0, 0.0).is_none());
        assert!(active_use_rate(1.0, 0.0).is_none());
    }
}

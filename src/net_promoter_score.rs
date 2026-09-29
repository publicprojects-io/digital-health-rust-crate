//! # Net Promoter Score
//!
//! Net Promoter Score (NPS) summarises patient satisfaction from a single
//! 0–10 "how likely are you to recommend..." question: respondents scoring
//! 9–10 are promoters, 0–6 detractors, and 7–8 passives. The score is the
//! promoter share minus the detractor share, on a scale from −100 to +100.
//! The survey response rate says how far the score can be trusted.
//!
//! ## How it's calculated
//!
//! ```text
//! NPS = (promoters − detractors) / respondents × 100
//!
//! Survey response rate = respondents / patients invited × 100
//! ```
//!
//! ## Why it matters
//!
//! NPS is the most widely used single-number satisfaction measure for
//! telehealth navigation, apps, and support layouts, and is easy to trend.
//! Unlike the other rates in this crate it is *not* a share in `[0, 100]`:
//! it ranges from −100 to +100, and a negative score is meaningful.
//!
//! ## Worked example
//!
//! A telehealth service sends its survey to 4,000 patients after a visit.
//! 1,000 respond: 600 promoters, 250 passives, and 150 detractors.
//!
//! ```rust
//! use digital_health::net_promoter_score::{net_promoter_score, survey_response_rate};
//!
//! // (600 − 150) / 1,000 × 100 = +45.
//! let nps = net_promoter_score(600.0, 150.0, 1_000.0).unwrap();
//! assert!((nps - 45.0).abs() < 1e-9);
//!
//! // 1,000 / 4,000 × 100 = 25% responded.
//! let response = survey_response_rate(1_000.0, 4_000.0).unwrap();
//! assert!((response - 25.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! The survey platform's response log supplies the counts. Passives count in
//! `respondents` but in neither the promoter nor detractor numerator.
//!
//! ## Pitfalls
//!
//! - **Ignoring the response rate** — a high NPS from a 3% response rate
//!   mostly reflects who chose to answer.
//! - **Comparing across survey timing or channel** — an in-app prompt right
//!   after a visit and an emailed survey a week later score differently.
//! - **Treating NPS as a clinical outcome** — satisfaction and clinical
//!   improvement are correlated at best, not interchangeable.
//!
//! ## Sources
//!
//! - Reichheld, F. F. "The One Number You Need to Grow." *Harvard Business
//!   Review*, 2003.
//! - Peer-reviewed literature on NPS use and validity in healthcare
//!   settings.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The Net Promoter Score: promoters minus detractors, as a percentage of
/// respondents. Ranges from −100 to +100.
///
/// # Arguments
///
/// * `promoters` — respondents scoring 9–10 (count).
/// * `detractors` — respondents scoring 0–6 (count).
/// * `respondents` — total survey respondents, including passives (count).
///
/// # Returns
///
/// `Some(score)`, or `None` if `respondents` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::net_promoter_score::net_promoter_score;
///
/// // Worked example: (600 − 150) / 1,000 × 100 = +45.
/// let nps = net_promoter_score(600.0, 150.0, 1_000.0).unwrap();
/// assert!((nps - 45.0).abs() < 1e-9);
///
/// assert!(net_promoter_score(1.0, 0.0, 0.0).is_none());
/// ```
#[must_use]
pub fn net_promoter_score(promoters: f64, detractors: f64, respondents: f64) -> Option<f64> {
    if respondents == 0.0 {
        None
    } else {
        Some((promoters - detractors) / respondents * 100.0)
    }
}

/// The survey response rate: respondents as a percentage of patients
/// invited to take the survey.
///
/// # Arguments
///
/// * `respondents` — patients who completed the survey (count).
/// * `invited` — patients the survey was sent to (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `invited` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::net_promoter_score::survey_response_rate;
///
/// // Worked example: 1,000 / 4,000 × 100 = 25%.
/// let rate = survey_response_rate(1_000.0, 4_000.0).unwrap();
/// assert!((rate - 25.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn survey_response_rate(respondents: f64, invited: f64) -> Option<f64> {
    if invited == 0.0 {
        None
    } else {
        Some(respondents / invited * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "600 promoters ... 150 detractors" of 1,000 — +45.
    #[test]
    fn worked_example_net_promoter_score_is_45() {
        let nps = net_promoter_score(600.0, 150.0, 1_000.0).unwrap();
        assert!((nps - 45.0).abs() < 1e-9, "got {nps}");
    }

    #[test]
    fn negative_score_is_preserved() {
        let nps = net_promoter_score(100.0, 300.0, 500.0).unwrap();
        assert!((nps + 40.0).abs() < 1e-9, "got {nps}");
    }

    // Doc line: "4,000 patients ... 1,000 respond" — 25%.
    #[test]
    fn worked_example_survey_response_rate_is_25_percent() {
        let rate = survey_response_rate(1_000.0, 4_000.0).unwrap();
        assert!((rate - 25.0).abs() < 1e-9, "got {rate}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(net_promoter_score(1.0, 0.0, 0.0).is_none());
        assert!(survey_response_rate(1.0, 0.0).is_none());
    }
}

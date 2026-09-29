//! # System Usability Scale
//!
//! The System Usability Scale (SUS) is a standardised 10-item questionnaire
//! that quantifies how usable patients or providers find software. Each
//! item is answered on a 1–5 agreement scale; the responses are converted
//! to a single score from 0 to 100. This module scores one respondent's
//! answers and averages scores across respondents.
//!
//! ## How it's calculated
//!
//! ```text
//! Odd-numbered items (1, 3, 5, 7, 9) contribute: response − 1
//! Even-numbered items (2, 4, 6, 8, 10) contribute: 5 − response
//!
//! SUS score = sum of the ten contributions × 2.5
//!
//! Mean SUS = sum of respondents' SUS scores / respondents
//! ```
//!
//! ## Why it matters
//!
//! A patient can be engaged with a tool they dislike, and a clinician can be
//! forced to use one. SUS is a short, free, widely benchmarked measure of
//! usability. A score of about 68 is commonly cited as the average across
//! products, so a score is best read against that benchmark rather than as a
//! percentage.
//!
//! ## Worked example
//!
//! One respondent answers items 1–10 as 5, 1, 5, 1, 4, 2, 5, 1, 4, 2. The
//! odd items contribute 18 and the even items 18, so the score is 36 × 2.5
//! = 90. Across 60 respondents,
//! individual scores sum to 4,080.
//!
//! ```rust
//! use digital_health::system_usability_scale::{mean_sus_score, sus_score};
//!
//! // (18 + 18) × 2.5 = 90.
//! let responses = [5.0, 1.0, 5.0, 1.0, 4.0, 2.0, 5.0, 1.0, 4.0, 2.0];
//! assert!((sus_score(responses) - 90.0).abs() < 1e-9);
//!
//! // 4,080 / 60 = 68.
//! let mean = mean_sus_score(4_080.0, 60.0).unwrap();
//! assert!((mean - 68.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Responses come from the standard SUS questionnaire with its original item
//! wording and order; reworded or reordered items are not comparable to the
//! published benchmarks. Every response must be a value from 1 to 5, in item
//! order, since this function does not validate its input.
//!
//! ## Pitfalls
//!
//! - **Averaging raw responses instead of scored items** — the even items
//!   are reverse-scored.
//! - **Reading SUS as a percentage** — a score of 68 is average, not 68%
//!   satisfied.
//! - **Mixing patient and provider respondents** — usability differs by
//!   role; report them separately.
//!
//! ## Sources
//!
//! - Brooke, J. "SUS: A Quick and Dirty Usability Scale." In *Usability
//!   Evaluation in Industry*, 1996.
//! - Bangor, A., Kortum, P. and Miller, J. "An Empirical Evaluation of the
//!   System Usability Scale." *International Journal of Human-Computer
//!   Interaction*, 2008.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The SUS score for one respondent: odd items contribute `response − 1`,
/// even items `5 − response`, and the sum is multiplied by 2.5.
///
/// # Arguments
///
/// * `responses` — the respondent's ten answers, in item order, each 1–5.
///
/// # Returns
///
/// The score from 0 to 100 for valid answers. Always defined — there is no
/// denominator.
///
/// # Examples
///
/// ```rust
/// use digital_health::system_usability_scale::sus_score;
///
/// // Worked example: (18 + 18) × 2.5 = 90.
/// let responses = [5.0, 1.0, 5.0, 1.0, 4.0, 2.0, 5.0, 1.0, 4.0, 2.0];
/// assert!((sus_score(responses) - 90.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn sus_score(responses: [f64; 10]) -> f64 {
    let contributions: f64 = responses
        .iter()
        .enumerate()
        .map(|(index, &response)| {
            // Index 0 is item 1, so even indices are the odd-numbered items.
            if index % 2 == 0 {
                response - 1.0
            } else {
                5.0 - response
            }
        })
        .sum();
    contributions * 2.5
}

/// The mean SUS score: the sum of respondents' individual scores divided by
/// the number of respondents.
///
/// # Arguments
///
/// * `total_scores` — the sum of every respondent's [`sus_score`].
/// * `respondents` — the number of respondents (count).
///
/// # Returns
///
/// `Some(mean)`, or `None` if `respondents` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::system_usability_scale::mean_sus_score;
///
/// // Worked example: 4,080 / 60 = 68.
/// let mean = mean_sus_score(4_080.0, 60.0).unwrap();
/// assert!((mean - 68.0).abs() < 1e-9);
///
/// assert!(mean_sus_score(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn mean_sus_score(total_scores: f64, respondents: f64) -> Option<f64> {
    if respondents == 0.0 {
        None
    } else {
        Some(total_scores / respondents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "answers ... 5, 1, 5, 1, 4, 2, 5, 1, 4, 2" — score 90.
    #[test]
    fn worked_example_sus_score_is_90() {
        let responses = [5.0, 1.0, 5.0, 1.0, 4.0, 2.0, 5.0, 1.0, 4.0, 2.0];
        assert!((sus_score(responses) - 90.0).abs() < 1e-9);
    }

    #[test]
    fn best_and_worst_possible_scores_are_100_and_0() {
        let best = [5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0];
        let worst = [1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0];
        assert!((sus_score(best) - 100.0).abs() < 1e-9);
        assert!(sus_score(worst).abs() < 1e-9);
    }

    // Doc line: "Across 60 respondents, individual scores sum to 4,080" — 68.
    #[test]
    fn worked_example_mean_sus_score_is_68() {
        let mean = mean_sus_score(4_080.0, 60.0).unwrap();
        assert!((mean - 68.0).abs() < 1e-9, "got {mean}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(mean_sus_score(1.0, 0.0).is_none());
    }
}

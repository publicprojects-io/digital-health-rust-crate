//! # Episode Cost Reduction
//!
//! Episode cost reduction measures the cost per episode of care — a
//! complete course of treatment for a condition — and the total saving when
//! a digital pathway lowers it against a historical baseline cohort.
//!
//! ## How it's calculated
//!
//! ```text
//! Cost per episode = total cost / episodes
//!
//! Total episode savings = (baseline cost per episode − current cost per episode) × episodes
//! ```
//!
//! ## Why it matters
//!
//! Value-based contracts are usually written per episode, not per visit, so
//! this is the unit a payer negotiates on. A per-episode figure normalises
//! for volume, and multiplying the per-episode difference by episode count
//! gives the programme-level saving.
//!
//! ## Worked example
//!
//! A digital musculoskeletal programme's 2,000 episodes cost an
//! illustrative $1,800,000.00 in total, so $900.00 each. The historical
//! baseline cohort cost $1,000.00 per episode.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::episode_cost_reduction::{cost_per_episode, total_episode_savings};
//!
//! // $1,800,000.00 / 2,000 = $900.00.
//! let cost_per_episode_value = cost_per_episode(Money::from_major(1_800_000, iso::USD), 2_000).unwrap();
//! assert_eq!(cost_per_episode_value, Money::from_major(900, iso::USD));
//!
//! // ($1,000.00 − $900.00) × 2,000 = $200,000.00.
//! let total_episode_savings_value = total_episode_savings(Money::from_major(1_000, iso::USD), Money::from_major(900, iso::USD), 2_000).unwrap();
//! assert_eq!(total_episode_savings_value, Money::from_major(200_000, iso::USD));
//!
//! ```
//!
//! ## Data sources and caveats
//!
//! Baseline and current episodes must share the same definition, case mix,
//! and cost-inclusion rules. Risk-adjust or match cohorts before
//! attributing the difference to the digital pathway, and include the
//! digital programme's own cost in the current-cost figure.
//!
//! ## Pitfalls
//!
//! - Comparing against a baseline with different case mix.
//! - Leaving the programme's own cost out of the current cost per episode.
//! - Dividing by an episode count that excludes episodes that were
//!   abandoned.
//!
//! ## Sources
//!
//! - Peer-reviewed literature on episode-based payment and bundled payment
//!   evaluation.
//! - CMS Bundled Payments for Care Improvement (BPCI) methodology.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The cost per episode of care: total cost divided by the number of
/// episodes.
///
/// # Arguments
///
/// * `total_cost` — total cost of the episodes, including the digital
///   programme.
/// * `episodes` — episodes of care completed (count).
///
/// # Errors
///
/// Returns [`MoneyError::DivisionByZero`] if `episodes` is zero, or
/// [`MoneyError::Overflow`] if the division overflows.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::episode_cost_reduction::cost_per_episode;
///
/// // Worked example: $1,800,000.00 / 2,000 = $900.00.
/// let value = cost_per_episode(Money::from_major(1_800_000, iso::USD), 2_000).unwrap();
/// assert_eq!(value, Money::from_major(900, iso::USD));
///
/// assert!(cost_per_episode(Money::from_major(1_000, iso::USD), 0).is_err());
/// ```
pub fn cost_per_episode(
    total_cost: Money<'static, iso::Currency>,
    episodes: u32,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    total_cost.div(episodes)
}

/// The total episode savings: the per-episode cost reduction against
/// baseline, multiplied by the number of episodes.
///
/// # Arguments
///
/// * `baseline_cost_per_episode` — historical baseline cost per episode.
/// * `current_cost_per_episode` — current cost per episode.
/// * `episodes` — episodes of care in the current period (count).
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if the two costs are not
/// denominated in the same currency, or [`MoneyError::Overflow`] if the
/// multiplication overflows.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::episode_cost_reduction::total_episode_savings;
///
/// // Worked example: ($1,000.00 − $900.00) × 2,000 = $200,000.00.
/// let value = total_episode_savings(Money::from_major(1_000, iso::USD), Money::from_major(900, iso::USD), 2_000).unwrap();
/// assert_eq!(value, Money::from_major(200_000, iso::USD));
///
/// assert!(total_episode_savings(Money::from_major(1_000, iso::USD), Money::from_major(900, iso::EUR), 2_000).is_err());
/// ```
pub fn total_episode_savings(
    baseline_cost_per_episode: Money<'static, iso::Currency>,
    current_cost_per_episode: Money<'static, iso::Currency>,
    episodes: u32,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    baseline_cost_per_episode
        .sub(current_cost_per_episode)?
        .mul(episodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "$1,800,000.00 / 2,000 = $900.00."
    #[test]
    fn worked_example_cost_per_episode() {
        let value = cost_per_episode(Money::from_major(1_800_000, iso::USD), 2_000).unwrap();
        assert_eq!(value, Money::from_major(900, iso::USD));
    }

    // Doc line: "($1,000.00 − $900.00) × 2,000 = $200,000.00."
    #[test]
    fn worked_example_total_episode_savings() {
        let value = total_episode_savings(
            Money::from_major(1_000, iso::USD),
            Money::from_major(900, iso::USD),
            2_000,
        )
        .unwrap();
        assert_eq!(value, Money::from_major(200_000, iso::USD));
    }

    #[test]
    fn zero_episodes_returns_err() {
        assert!(cost_per_episode(Money::from_major(1_000, iso::USD), 0).is_err());
    }

    #[test]
    fn total_episode_savings_currency_mismatch_returns_err() {
        assert!(
            total_episode_savings(
                Money::from_major(1_000, iso::USD),
                Money::from_major(900, iso::EUR),
                2_000
            )
            .is_err()
        );
    }
}

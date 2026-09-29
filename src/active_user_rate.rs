//! # Active User Rate
//!
//! Active user rate is the baseline engagement measure for a digital care
//! portal or app: the share of enrolled users active in a period (weekly
//! for WAU, daily for DAU), and the stickiness ratio — daily active users
//! as a share of monthly active users.
//!
//! ## How it's calculated
//!
//! ```text
//! Active user rate = active users / enrolled users × 100
//!
//! Stickiness = daily active users / monthly active users × 100
//! ```
//!
//! ## Why it matters
//!
//! Nothing else in a digital programme matters if patients don't show up.
//! Active user rate shows reach, and stickiness shows habit: a product with
//! many monthly users but few daily ones is visited, not used. It sits
//! upstream of
//! [`engagement_consistency_rate`](crate::engagement_consistency_rate).
//!
//! ## Worked example
//!
//! A care app has 8,000 enrolled patients, of whom 3,200 were active in the
//! last week. It has 6,000 monthly active users, and on an average day
//! 1,800 are active.
//!
//! ```rust
//! use digital_health::active_user_rate::{active_user_rate, stickiness_ratio};
//!
//! // 3,200 / 8,000 × 100 = 40% weekly active.
//! let active_user_rate_value = active_user_rate(3_200.0, 8_000.0).unwrap();
//! assert!((active_user_rate_value - 40.0).abs() < 1e-9);
//!
//! // 1,800 / 6,000 × 100 = 30% stickiness.
//! let stickiness_ratio_value = stickiness_ratio(1_800.0, 6_000.0).unwrap();
//! assert!((stickiness_ratio_value - 30.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Define "active" as a meaningful action, not an app open, and hold it
//! constant. For stickiness, use the average daily actives over the month,
//! and the same definition for monthly actives.
//!
//! ## Pitfalls
//!
//! - Counting opens or logins as activity.
//! - Using total registered users as the denominator — enrolled and still
//!   eligible is fairer.
//! - Comparing stickiness across products with different natural usage
//!   frequency.
//!
//! ## Sources
//!
//! - Product-analytics literature on DAU/MAU stickiness.
//! - Peer-reviewed literature on engagement measurement in digital health
//!   interventions.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The active user rate: users active in the period as a percentage of
/// enrolled users.
///
/// # Arguments
///
/// * `active_users` — users with a meaningful action in the period (count).
/// * `enrolled_users` — users enrolled and eligible (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `enrolled_users` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::active_user_rate::active_user_rate;
///
/// // Worked example: 3,200 / 8,000 × 100 = 40%.
/// let value = active_user_rate(3_200.0, 8_000.0).unwrap();
/// assert!((value - 40.0).abs() < 1e-9);
///
/// assert!(active_user_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn active_user_rate(active_users: f64, enrolled_users: f64) -> Option<f64> {
    if enrolled_users == 0.0 {
        None
    } else {
        Some(active_users / enrolled_users * 100.0)
    }
}

/// The stickiness ratio: average daily active users as a percentage of
/// monthly active users.
///
/// # Arguments
///
/// * `daily_active_users` — average daily active users over the month.
/// * `monthly_active_users` — distinct users active in the month.
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `monthly_active_users` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::active_user_rate::stickiness_ratio;
///
/// // Worked example: 1,800 / 6,000 × 100 = 30%.
/// let value = stickiness_ratio(1_800.0, 6_000.0).unwrap();
/// assert!((value - 30.0).abs() < 1e-9);
///
/// assert!(stickiness_ratio(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn stickiness_ratio(daily_active_users: f64, monthly_active_users: f64) -> Option<f64> {
    if monthly_active_users == 0.0 {
        None
    } else {
        Some(daily_active_users / monthly_active_users * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "3,200 / 8,000 × 100 = 40% weekly active."
    #[test]
    fn worked_example_active_user_rate() {
        let value = active_user_rate(3_200.0, 8_000.0).unwrap();
        assert!((value - 40.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "1,800 / 6,000 × 100 = 30% stickiness."
    #[test]
    fn worked_example_stickiness_ratio() {
        let value = stickiness_ratio(1_800.0, 6_000.0).unwrap();
        assert!((value - 30.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(active_user_rate(1.0, 0.0).is_none());
        assert!(stickiness_ratio(1.0, 0.0).is_none());
    }
}

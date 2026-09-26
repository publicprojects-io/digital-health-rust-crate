//! # No-Show Lost Revenue
//!
//! No-show lost revenue puts a currency amount on [appointment no-show
//! rate](crate::appointment_no_show_rate): the revenue a scheduled
//! appointment would have generated, multiplied by the count of true
//! no-shows, and the resulting gap against gross scheduled revenue. Where
//! the rate tells an operations team *how often* capacity is lost, this
//! module tells a finance team *how much* — the number that turns a
//! reminders-and-rescheduling investment case into a currency figure
//! leadership can weigh against its cost.
//!
//! ## How it's calculated
//!
//! ```text
//! Lost revenue = no-shows × revenue per appointment
//!
//! Net revenue impact = gross scheduled revenue − lost revenue
//! ```
//!
//! ## Why it matters
//!
//! [Appointment no-show rate](crate::appointment_no_show_rate) is the
//! operational signal; this module converts it into the currency terms a
//! business case needs. A digital reminder or self-service rescheduling
//! programme has a cost, and lost revenue is the number that programme is
//! justified against. Because rates alone don't reveal magnitude — a 9%
//! no-show rate means something very different at $50 versus $500 per
//! appointment — the currency figure has to be calculated separately from
//! the rate, not inferred from it.
//!
//! ## Worked example
//!
//! Continuing [appointment no-show rate](crate::appointment_no_show_rate)'s
//! worked example: a community clinic schedules 2,000 appointments in a
//! month, of which 180 are true no-shows, at an illustrative $150.00
//! reimbursement per completed appointment.
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use digital_health::no_show_lost_revenue::{lost_revenue, net_revenue_impact};
//!
//! let revenue_per_appointment = Money::from_major(150, iso::USD);
//!
//! // 180 no-shows × $150.00 = $27,000.00.
//! let lost = lost_revenue(180, revenue_per_appointment).unwrap();
//! assert_eq!(lost, Money::from_major(27_000, iso::USD));
//!
//! // Had all 2,000 appointments completed, gross revenue would have been
//! // $300,000.00; net of the no-shows, it's $273,000.00.
//! let gross_scheduled = Money::from_major(300_000, iso::USD);
//! let net = net_revenue_impact(gross_scheduled, lost).unwrap();
//! assert_eq!(net, Money::from_major(273_000, iso::USD));
//! ```
//!
//! ## Data sources and caveats
//!
//! `revenue_per_appointment` should reflect the actual expected
//! reimbursement or self-pay revenue for the appointment type being
//! measured (it varies enormously by payer, specialty, and appointment
//! type), not a clinic-wide average — blending appointment types into one
//! average revenue figure will misstate the lost-revenue calculation for
//! any specific service line.
//!
//! ## Pitfalls
//!
//! - **Using a blended average revenue figure across appointment types** —
//!   a no-show in a high-reimbursement specialty slot costs far more than
//!   one in a low-reimbursement one; calculate lost revenue per appointment
//!   type where the underlying rate is segmented that way.
//! - **Treating lost revenue as fully recoverable** — a reduced no-show rate
//!   only recovers this revenue if the freed capacity is actually rebooked;
//!   an empty slot that goes unfilled either way recovers nothing.
//! - **Ignoring marginal versus average cost** — the *marginal* revenue lost
//!   to a no-show (what the clinic would have earned from the visit that
//!   didn't happen) is not the same as the *average* revenue per
//!   appointment if the clinic is capacity-constrained and would have
//!   double-booked that slot regardless.
//!
//! ## Sources
//!
//! - See [appointment no-show rate](crate::appointment_no_show_rate)'s
//!   `## Sources` for the underlying rate's evidence base; this module adds
//!   only the currency arithmetic on top.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

use rusty_money::{Money, MoneyError, iso};

/// The lost revenue from no-shows: the count of true no-shows multiplied by
/// the revenue a single completed appointment would have generated.
///
/// # Arguments
///
/// * `no_shows` — appointments recorded as "did not attend" (count).
/// * `revenue_per_appointment` — expected revenue for one completed
///   appointment of the type being measured.
///
/// # Errors
///
/// Returns [`MoneyError::Overflow`] if the multiplication overflows
/// `revenue_per_appointment`'s underlying decimal representation. Does not
/// return a currency-mismatch error: multiplying by a plain count can't
/// mismatch currencies.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::no_show_lost_revenue::lost_revenue;
///
/// // Worked example: 180 × $150.00 = $27,000.00.
/// let revenue_per_appointment = Money::from_major(150, iso::USD);
/// let lost = lost_revenue(180, revenue_per_appointment).unwrap();
/// assert_eq!(lost, Money::from_major(27_000, iso::USD));
/// ```
pub fn lost_revenue(
    no_shows: u32,
    revenue_per_appointment: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    revenue_per_appointment.mul(no_shows)
}

/// The net revenue impact: gross scheduled revenue minus the revenue lost
/// to no-shows.
///
/// # Arguments
///
/// * `gross_scheduled_revenue` — revenue that would have resulted had every
///   scheduled appointment completed.
/// * `lost_revenue` — the revenue lost to no-shows, typically the result of
///   [`lost_revenue`].
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `gross_scheduled_revenue` and
/// `lost_revenue` are not denominated in the same currency.
///
/// # Examples
///
/// ```rust
/// use rusty_money::{Money, iso};
/// use digital_health::no_show_lost_revenue::net_revenue_impact;
///
/// // Worked example: $300,000.00 − $27,000.00 = $273,000.00.
/// let gross_scheduled = Money::from_major(300_000, iso::USD);
/// let lost = Money::from_major(27_000, iso::USD);
/// let net = net_revenue_impact(gross_scheduled, lost).unwrap();
/// assert_eq!(net, Money::from_major(273_000, iso::USD));
///
/// let lost_eur = Money::from_major(27_000, iso::EUR);
/// assert!(net_revenue_impact(gross_scheduled, lost_eur).is_err());
/// ```
pub fn net_revenue_impact(
    gross_scheduled_revenue: Money<'static, iso::Currency>,
    lost_revenue: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    gross_scheduled_revenue.sub(lost_revenue)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "180 no-shows × $150.00 = $27,000.00."
    #[test]
    fn worked_example_lost_revenue_is_27_000_usd() {
        let revenue_per_appointment = Money::from_major(150, iso::USD);
        let lost = lost_revenue(180, revenue_per_appointment).unwrap();
        assert_eq!(lost, Money::from_major(27_000, iso::USD));
    }

    // Doc line: "net of the no-shows, it's $273,000.00."
    #[test]
    fn worked_example_net_revenue_impact_is_273_000_usd() {
        let revenue_per_appointment = Money::from_major(150, iso::USD);
        let lost = lost_revenue(180, revenue_per_appointment).unwrap();
        let gross = Money::from_major(300_000, iso::USD);
        let net = net_revenue_impact(gross, lost).unwrap();
        assert_eq!(net, Money::from_major(273_000, iso::USD));
    }

    #[test]
    fn currency_mismatch_returns_err() {
        let gross = Money::from_major(300_000, iso::USD);
        let lost_eur = Money::from_major(27_000, iso::EUR);
        assert!(net_revenue_impact(gross, lost_eur).is_err());
    }
}

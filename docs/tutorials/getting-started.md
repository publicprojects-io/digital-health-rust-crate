# Getting started

This walks through using `digital-health` end to end: installing it,
running the portal-adoption funnel and no-show rate from the crate's own
quickstart, a second worked example — the digital-referral
turnaround-time percentiles — that the quickstart doesn't cover, and a
third showing the crate's `Money`-based cost-and-revenue functions.

## Install

```toml
[dependencies]
digital-health = "1.0"
```

## The portal-adoption funnel and the no-show rate

A primary care network serves 50,000 eligible patients; 32,000 have
registered for the portal, and 21,000 logged in at least once in the last
12 months. The same clinic scheduled 2,000 appointments this month, of
which 180 were true no-shows.

```rust
use digital_health::patient_portal_adoption_rate as portal;
use digital_health::appointment_no_show_rate as no_show;

let registration = portal::registration_rate(32_000.0, 50_000.0).unwrap();
let active_use = portal::active_use_rate(21_000.0, 50_000.0).unwrap();
assert!((registration - 64.0).abs() < 1e-9);
assert!((active_use - 42.0).abs() < 1e-9);

// Reporting registration alone would considerably overstate real engagement.
assert!(active_use < registration);

let rate = no_show::no_show_rate(180.0, 2_000.0).unwrap();
assert!((rate - 9.0).abs() < 1e-9);
```

`registration_rate` and `active_use_rate` both divide by the same
denominator (`eligible`), which is why they're directly comparable in that
last assertion. See
[`spec/patient-portal-adoption-rate.md`](../../spec/patient-portal-adoption-rate.md)
for the full contract, including where `activation_rate` divides by a
*different* denominator (`registered`, not `eligible`).

## A second example: referral turnaround percentiles

An e-referral system logs ten turnaround times (in days) for a month. The
operational question is rarely "what's the average" — a right-skewed
distribution makes the mean misleading — so report the median and the 90th
percentile instead:

```rust
use digital_health::digital_referral_turnaround_time::{percentile, hours_to_days};

let durations = [0.3, 0.5, 0.8, 1.2, 1.8, 1.8, 2.5, 3.5, 6.0, 6.0];
let median = percentile(&durations, 50.0).unwrap();
let p90 = percentile(&durations, 90.0).unwrap();
assert!((median - 1.8).abs() < 1e-9);
assert!((p90 - 6.0).abs() < 1e-9);

// A fast pathway (e.g. image-based teledermatology triage) reported in
// hours, converted to the same unit as the durations above for comparison.
let fast_pathway_days = hours_to_days(4.0);
assert!(fast_pathway_days < median);
```

`percentile` never panics — even on an empty slice (it returns `None`) or on
`NaN` input (it sorts with a total order). See
[`spec/digital-referral-turnaround-time.md`](../../spec/digital-referral-turnaround-time.md).

## A third example: putting a currency amount on a rate

Every module so far takes and returns `f64`. The two cost-and-revenue
modules instead take `u32` counts and a
[`rusty_money::Money`](https://docs.rs/rusty-money) amount, returning
`Result<Money, MoneyError>` — continuing the no-show example above, at an
illustrative $150.00 per completed appointment:

```rust
use rusty_money::{Money, iso};
use digital_health::no_show_lost_revenue::{lost_revenue, net_revenue_impact};

let revenue_per_appointment = Money::from_major(150, iso::USD);

// 180 no-shows × $150.00 = $27,000.00.
let lost = lost_revenue(180, revenue_per_appointment).unwrap();
assert_eq!(lost, Money::from_major(27_000, iso::USD));

// Had all 2,000 appointments completed, gross revenue would have been
// $300,000.00; net of the no-shows, it's $273,000.00.
let gross_scheduled = Money::from_major(300_000, iso::USD);
let net = net_revenue_impact(gross_scheduled, lost).unwrap();
assert_eq!(net, Money::from_major(273_000, iso::USD));
```

`u32` counts convert to `rusty_money`'s decimal type exactly — there's no
`f64`-to-decimal precision-loss question to worry about. Mixing currencies
(e.g. subtracting a EUR amount from a USD one) returns `Err`, never a wrong
answer or a panic. See
[`spec/no-show-lost-revenue.md`](../../spec/no-show-lost-revenue.md) and
`spec/README.md`'s "Money convention" section.

## Next steps

- Run `cargo doc --open` — every module's rustdoc has the same shape: what
  the metric is, how it's calculated, why it matters, a worked example, data
  sources, and pitfalls.
- Read [`spec/README.md`](../../spec/README.md) if you're checking a
  formula against its contract rather than its prose explanation.
- See
  [`adding-a-metric.md`](adding-a-metric.md) if you're extending the crate
  with a new module.

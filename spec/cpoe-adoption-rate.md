# Spec: CPOE Adoption Rate

- **Module**: [`src/cpoe_adoption_rate.rs`](../src/cpoe_adoption_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `electronic_order_rate(electronic_orders: f64, total_orders: f64) -> Option<f64>`

- Formula: `electronic_orders / total_orders * 100`
- Returns `None` iff `total_orders == 0.0`
- Worked example: `electronic_order_rate(9_400.0, 10_000.0) == Some(94.0)`

### `verbal_order_rate(verbal_orders: f64, total_orders: f64) -> Option<f64>`

- Formula: `verbal_orders / total_orders * 100`
- Returns `None` iff `total_orders == 0.0`
- Worked example: `verbal_order_rate(500.0, 10_000.0) == Some(5.0)`

## Invariants

- The two functions share the same denominator (`total_orders`) when called
  on the same period/order-type slice, so `electronic_order_rate(a, d) +
  verbal_order_rate(b, d)` is a meaningful partial sum only when `a` and `b`
  are non-overlapping order counts from that same slice; neither function
  itself enforces that.

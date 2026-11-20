//! Data-integrity tests: metric inputs and outputs survive a JSON round trip.
//!
//! The crate itself has no serialization; `serde_json` is a dev-dependency
//! used only here. `Option<f64>` results must keep `None` distinct from any
//! number (JSON `null`), and finite `f64` values must round-trip exactly.

use digital_health::{
    net_promoter_score::net_promoter_score, readmission_rate::readmission_rate,
    system_usability_scale::sus_score,
};

#[test]
fn some_result_round_trips_exactly() {
    let result = readmission_rate(400.0, 2_500.0);
    let json = serde_json::to_string(&result).unwrap();
    let back: Option<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(result, back);
}

#[test]
fn none_serializes_as_null_and_back() {
    let result = readmission_rate(1.0, 0.0);
    assert_eq!(result, None);
    let json = serde_json::to_string(&result).unwrap();
    assert_eq!(json, "null");
    let back: Option<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(back, None);
}

#[test]
fn inputs_round_trip_before_calculation() {
    let inputs = vec![60.0_f64, 15.0, 100.0];
    let json = serde_json::to_string(&inputs).unwrap();
    let back: Vec<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(inputs, back);
    assert_eq!(
        net_promoter_score(inputs[0], inputs[1], inputs[2]),
        net_promoter_score(back[0], back[1], back[2]),
    );
}

#[test]
fn awkward_fractions_round_trip_bit_exactly() {
    for x in [0.1, 1.0 / 3.0, 2.0_f64.sqrt(), 1e-300, 1e300, -0.0] {
        let json = serde_json::to_string(&x).unwrap();
        let back: f64 = serde_json::from_str(&json).unwrap();
        assert_eq!(x.to_bits(), back.to_bits(), "{x} via {json}");
    }
}

#[test]
fn array_argument_round_trips() {
    let responses = [4.0, 2.0, 4.0, 2.0, 5.0, 1.0, 5.0, 1.0, 4.0, 2.0];
    let json = serde_json::to_string(&responses).unwrap();
    let back: [f64; 10] = serde_json::from_str(&json).unwrap();
    assert_eq!(responses, back);
    assert!((sus_score(back) - sus_score(responses)).abs() < f64::EPSILON);
}

#[test]
fn non_finite_results_are_not_silently_serialized_as_numbers() {
    // serde_json writes NaN/inf as `null`; the crate returns `None` instead of
    // producing them, so a zero denominator never reaches this lossy path.
    assert_eq!(serde_json::to_string(&f64::NAN).unwrap(), "null");
    assert_eq!(readmission_rate(f64::NAN, 0.0), None);
}

mod money {
    //! `Money` is serialized as an (amount string, ISO currency code) pair so
    //! the decimal amount never passes through `f64`.

    use digital_health::bed_day_cost_avoidance::bed_day_cost_avoided;
    use rusty_money::{Money, iso};
    use serde_json::json;

    #[test]
    fn money_result_round_trips_without_precision_loss() {
        let cost = bed_day_cost_avoided(300, Money::from_minor(50_033, iso::USD)).unwrap();

        let value = json!({
            "amount": cost.amount().to_string(),
            "currency": cost.currency().iso_alpha_code,
        });
        let text = serde_json::to_string(&value).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();

        assert_eq!(parsed["currency"], "USD");
        let code = parsed["currency"].as_str().unwrap();
        let currency = iso::find(code).unwrap();
        let back = Money::from_str(parsed["amount"].as_str().unwrap(), currency).unwrap();
        assert_eq!(back, cost);
        // 300 × $500.33 = $150,099.00 exactly; an f64 path could not promise that.
        assert_eq!(back, Money::from_minor(15_009_900, iso::USD));
    }

    #[test]
    fn currency_survives_round_trip() {
        let eur = Money::from_major(10, iso::EUR);
        let code = eur.currency().iso_alpha_code;
        assert_eq!(*iso::find(code).unwrap(), *eur.currency());
    }
}

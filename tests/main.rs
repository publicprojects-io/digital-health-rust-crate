//! Integration tests, folded into one test binary.

#[path = "main/cross_validation.rs"]
mod cross_validation;
#[path = "main/reference_validation.rs"]
mod reference_validation;
#[path = "main/serialization.rs"]
mod serialization;

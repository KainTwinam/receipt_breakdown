
pub fn validate_i64(input: &str) -> bool {
    input.parse::<i64>().is_ok()
}

pub fn convert_to_i64(input: &str) -> i64 {
    input.parse::<i64>().unwrap_or(0)
}

pub fn validate_f64(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

pub fn convert_to_f64(input: &str) -> f64 {
    input.parse::<f64>().unwrap_or(0.0)
}
use std::fmt;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Quantity(pub i32);

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Use an alternate format if the `#` flag is present
            write!(f, "Quantity: {}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
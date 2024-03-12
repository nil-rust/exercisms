use std::ops::Add;
use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::new(1000000000, 0);

    start.add(gigasecond)
}

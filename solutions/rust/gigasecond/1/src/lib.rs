use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let billion_seconds = Duration::new(1_000_000_000, 0);
    return start.checked_add(billion_seconds).unwrap();
}

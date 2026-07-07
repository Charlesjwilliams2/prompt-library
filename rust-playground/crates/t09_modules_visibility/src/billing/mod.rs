// TODO(reader): `mod invoice;` below pulls in `billing/invoice.rs` as a
// child module. Nothing in this file is visible outside `billing` unless
// it's re-exported or declared `pub`.

mod invoice;

pub use invoice::Invoice;

// `pub(crate)`: visible anywhere in this crate, but not to other crates
// that might depend on this one as a library. Since this is a `[[bin]]`
// crate there's no "other crate" to hide it from in practice, but the
// annotation documents intent for whoever reads this module.
pub(crate) fn tax_rate() -> f64 {
    0.08
}

// Private to `billing` (and its children, via `super::`) — nothing outside
// this module can call it directly.
fn round_to_cents(amount: f64) -> f64 {
    (amount * 100.0).round() / 100.0
}

pub fn total_with_tax(subtotal_cents: u32) -> f64 {
    let subtotal = subtotal_cents as f64 / 100.0;
    round_to_cents(subtotal * (1.0 + tax_rate()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_with_tax_applies_tax_rate() {
        assert_eq!(total_with_tax(10_000), 10800.0 / 100.0);
    }
}

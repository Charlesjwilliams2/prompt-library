// This is `billing::invoice`, a child of `billing`. It can see `billing`'s
// private items through `super::`, but nothing outside `billing` can see
// into here unless `billing/mod.rs` re-exports it (which it does, via
// `pub use invoice::Invoice;`).

pub struct Invoice {
    pub customer: String,
    subtotal_cents: u32,
}

impl Invoice {
    pub fn new(customer: impl Into<String>, subtotal_cents: u32) -> Self {
        Self {
            customer: customer.into(),
            subtotal_cents,
        }
    }

    pub fn total_with_tax(&self) -> f64 {
        // Calls a private-to-`billing` function through `super::` — this
        // only compiles because `invoice.rs` is a child module of `billing`.
        super::total_with_tax(self.subtotal_cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoice_total_includes_tax() {
        let invoice = Invoice::new("Test Customer", 10_000);
        assert_eq!(invoice.total_with_tax(), 108.0);
    }
}

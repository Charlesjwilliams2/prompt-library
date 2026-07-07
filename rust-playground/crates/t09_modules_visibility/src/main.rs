// TODO(reader): before reading further, look at `src/billing/mod.rs` and
// `src/billing/invoice.rs`. Draw (on paper, or in your head) which items
// are visible from `main.rs`, and which are only visible inside `billing`.

mod billing;

use billing::Invoice;

fn main() {
    let invoice = Invoice::new("Rowan", 12_345);
    println!("{}: ${:.2}", invoice.customer, invoice.total_with_tax());

    // TODO(reader): uncomment either line below to see a "private item"
    // compiler error, then comment it back out.
    // println!("{}", billing::tax_rate());       // pub(crate) — try it anyway, does it work?
    // println!("{}", billing::round_to_cents(1.005)); // not `pub` at all — this should fail
}

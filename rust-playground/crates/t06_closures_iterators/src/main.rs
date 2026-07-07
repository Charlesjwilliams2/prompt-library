// TODO(reader): iterator chains read right-to-left in effect but
// left-to-right on the page. As you read each `.method()`, ask what shape
// of data goes in and what shape comes out.

#[derive(Debug, Clone)]
struct Order {
    customer: String,
    total_cents: u32,
    fulfilled: bool,
}

fn sample_orders() -> Vec<Order> {
    vec![
        Order {
            customer: "amina".into(),
            total_cents: 4200,
            fulfilled: true,
        },
        Order {
            customer: "bo".into(),
            total_cents: 900,
            fulfilled: false,
        },
        Order {
            customer: "chidi".into(),
            total_cents: 15000,
            fulfilled: true,
        },
        Order {
            customer: "dana".into(),
            total_cents: 300,
            fulfilled: false,
        },
    ]
}

fn main() {
    let orders = sample_orders();

    // A closure capturing nothing, used as the predicate for `filter`.
    let is_fulfilled = |o: &&Order| o.fulfilled;

    let fulfilled_total: u32 = orders
        .iter()
        .filter(is_fulfilled)
        .map(|o| o.total_cents)
        .sum();
    println!("fulfilled total: {fulfilled_total} cents");

    // A closure that captures `threshold` by reference from the
    // surrounding scope (a "move" isn't needed since `threshold` is Copy).
    let threshold = 1000;
    let big_spenders: Vec<&str> = orders
        .iter()
        .filter(|o| o.total_cents > threshold)
        .map(|o| o.customer.as_str())
        .collect();
    println!("big spenders: {big_spenders:?}");

    // `fold` builds up a value across the whole iterator — useful when
    // there's no built-in adapter (like `sum`/`max`) for what you need.
    let summary = orders.iter().fold(String::new(), |mut acc, o| {
        acc.push_str(&format!("{}:{} ", o.customer, o.total_cents));
        acc
    });
    println!("summary: {}", summary.trim());

    // Equivalent loop, for comparison with the iterator version above.
    let mut manual_total = 0u32;
    for order in &orders {
        if order.fulfilled {
            manual_total += order.total_cents;
        }
    }
    assert_eq!(manual_total, fulfilled_total);

    // `Fn` vs `FnMut`: this closure mutates a captured variable, so it
    // must be called through something that allows `FnMut`, not `Fn`.
    let mut call_count = 0;
    let mut counting_double = |x: u32| {
        call_count += 1;
        x * 2
    };
    let doubled: Vec<u32> = orders
        .iter()
        .map(|o| counting_double(o.total_cents))
        .collect();
    println!("doubled: {doubled:?}, closure called {call_count} times");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fulfilled_total_matches_manual_sum() {
        let orders = sample_orders();
        let fulfilled_total: u32 = orders
            .iter()
            .filter(|o| o.fulfilled)
            .map(|o| o.total_cents)
            .sum();
        assert_eq!(fulfilled_total, 4200 + 15000);
    }

    #[test]
    fn big_spenders_above_threshold() {
        let orders = sample_orders();
        let names: Vec<&str> = orders
            .iter()
            .filter(|o| o.total_cents > 1000)
            .map(|o| o.customer.as_str())
            .collect();
        assert_eq!(names, vec!["amina", "chidi"]);
    }
}

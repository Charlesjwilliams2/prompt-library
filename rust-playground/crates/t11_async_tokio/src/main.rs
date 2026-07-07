// TODO(reader): an `async fn` doesn't run until something `.await`s it (or
// a runtime polls it). As you read, ask: where does the runtime actually
// come from, and which calls could block the whole runtime if misused?

use std::time::Duration;
use tokio::time::sleep;

// An `async fn` compiles to a state machine; calling it just builds that
// state machine and returns immediately. Nothing here runs until awaited.
async fn fetch_price(item: &str, delay_ms: u64) -> (String, u32) {
    sleep(Duration::from_millis(delay_ms)).await;
    let price = (item.len() as u32) * 100;
    (item.to_string(), price)
}

// This function is deliberately wrong: `std::thread::sleep` blocks the
// current OS thread instead of yielding control back to the async
// runtime, which can stall every other task scheduled on that thread.
async fn fetch_price_blocking(item: &str, delay_ms: u64) -> (String, u32) {
    // TODO(reader): this line is the bug. Compare it to `fetch_price` above.
    std::thread::sleep(Duration::from_millis(delay_ms));
    let price = (item.len() as u32) * 100;
    (item.to_string(), price)
}

#[tokio::main]
async fn main() {
    // `tokio::join!` runs both futures concurrently on the current task,
    // rather than awaiting them one after another.
    let (apple, banana) = tokio::join!(fetch_price("apple", 30), fetch_price("banana", 10),);
    println!("{apple:?}, {banana:?}");

    // `tokio::spawn` hands a future to the runtime to run on its own task,
    // returning a `JoinHandle` you must `.await` to get the result back.
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(20)).await;
        "spawned task finished"
    });
    let result = handle.await.unwrap();
    println!("{result}");

    // This call compiles and produces a correct-looking result, but the
    // blocking `std::thread::sleep` inside it means the runtime's worker
    // thread is stuck for the full duration instead of running other
    // ready tasks in the meantime.
    let (carrot, _price) = fetch_price_blocking("carrot", 15).await;
    println!("{carrot} (fetched, but see WORKBOOK.md about how)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_price_scales_with_item_name_length() {
        let (item, price) = fetch_price("apple", 1).await;
        assert_eq!(item, "apple");
        assert_eq!(price, 500);
    }

    #[tokio::test]
    async fn join_runs_futures_concurrently() {
        let start = std::time::Instant::now();
        tokio::join!(fetch_price("a", 30), fetch_price("b", 30));
        // If these ran sequentially this would take ~60ms; concurrently it
        // should take roughly one delay's worth, not the sum of both.
        assert!(start.elapsed() < Duration::from_millis(55));
    }
}

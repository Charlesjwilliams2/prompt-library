// TODO(reader): this is the capstone. Nothing here is new — every pattern
// below reuses a topic from crates 01-12. Before reading WORKBOOK.md, go
// through this file and, for each function, name which earlier topic's
// checklist it belongs to and what you'd change.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Smell: everything wrapped in `Rc<RefCell<T>>` despite being used on one
// thread with a single owner. Compare to topic 07.
struct Cache {
    entries: Rc<RefCell<Vec<(String, u32)>>>,
}

impl Cache {
    fn new() -> Self {
        Self {
            entries: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn insert(&self, key: &str, value: u32) {
        self.entries.borrow_mut().push((key.to_string(), value));
    }

    fn get(&self, key: &str) -> Option<u32> {
        self.entries
            .borrow()
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v)
    }
}

// Smell: `.unwrap()` on every fallible operation instead of propagating
// with `?`. Compare to topic 03.
fn parse_all(values: &[&str]) -> Vec<i32> {
    values.iter().map(|v| v.parse::<i32>().unwrap()).collect()
}

// Smell: needless `.clone()` calls where a borrow would do. Compare to
// topics 01 and 12.
fn longest_name(names: &[String]) -> String {
    let mut longest = names[0].clone();
    for name in names {
        if name.clone().len() > longest.clone().len() {
            longest = name.clone();
        }
    }
    longest
}

// Smell: `Arc<Mutex<T>>` used with no threads anywhere near it. Compare to
// topic 07.
fn count_vowels_unnecessarily_locked(text: &str) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    for c in text.chars() {
        if "aeiouAEIOU".contains(c) {
            *counter.lock().unwrap() += 1;
        }
    }
    *counter.lock().unwrap()
}

// Smell: an overengineered generic signature for a function only ever
// called with one concrete type. Compare to topic 05.
fn print_first<T: Clone + std::fmt::Debug + PartialEq + Send + Sync>(items: &[T]) {
    if let Some(first) = items.first() {
        println!("{first:?}");
    }
}

fn main() {
    let cache = Cache::new();
    cache.insert("pi", 3);
    println!("cached: {:?}", cache.get("pi"));

    println!("{:?}", parse_all(&["1", "2", "3"]));

    let names = vec![
        "Al".to_string(),
        "Bartholomew".to_string(),
        "Cy".to_string(),
    ];
    println!("longest: {}", longest_name(&names));

    println!(
        "vowels: {}",
        count_vowels_unnecessarily_locked("hello world")
    );

    print_first(&[1, 2, 3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_get_returns_inserted_value() {
        let cache = Cache::new();
        cache.insert("pi", 3);
        assert_eq!(cache.get("pi"), Some(3));
        assert_eq!(cache.get("missing"), None);
    }

    #[test]
    fn longest_name_picks_the_longest() {
        let names = vec![
            "Al".to_string(),
            "Bartholomew".to_string(),
            "Cy".to_string(),
        ];
        assert_eq!(longest_name(&names), "Bartholomew");
    }
}

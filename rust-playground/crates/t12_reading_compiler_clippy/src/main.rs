// TODO(reader): this crate builds cleanly with `cargo build`, but
// `cargo clippy` should flag several lines below. That's the point —
// clippy catches things the compiler considers perfectly valid Rust.
// Run `cargo clippy -p t12_reading_compiler_clippy` before reading
// WORKBOOK.md and see how many of the flagged lines you can find first.

fn describe_count(count: u32) -> String {
    // clippy: `count == 0` reads better as a direct bool check than
    // comparing to a literal `true`/`false`, but this specific comparison
    // isn't the flagged line — see below.
    let is_empty = count == 0;
    // clippy::bool_comparison — comparing a bool to a literal `true`.
    if is_empty == true {
        "nothing here".to_string()
    } else {
        format!("{count} items")
    }
}

fn total_length(items: &[String]) -> usize {
    // `items` is already borrowed; cloning each `String` just to read its
    // length is wasted work when `.len()` works fine on a reference. This
    // one, notably, does *not* get flagged by clippy's default lints — you
    // have to catch it by reading, not by waiting for a warning.
    items.iter().map(|s| s.clone().len()).sum()
}

fn find_first_admin<'a>(users: &[(&'a str, &str)]) -> Option<&'a str> {
    // clippy::single_match — a `match` with only one meaningful arm and a
    // `_ => None` fallback is usually clearer as `if let`.
    for (name, role) in users {
        match *role {
            "admin" => return Some(name),
            _ => {}
        }
    }
    None
}

fn main() {
    println!("{}", describe_count(0));
    println!("{}", describe_count(3));

    let items = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
    println!("total length: {}", total_length(&items));

    let users = [("alice", "member"), ("bo", "admin")];
    println!("first admin: {:?}", find_first_admin(&users));

    // TODO(reader): uncomment this block to produce a real E0382 "use of
    // moved value" compiler error (not a clippy lint — an actual error
    // that stops the build). Run `cargo build -p t12_reading_compiler_clippy`,
    // read the error and the line/column it points to, then comment the
    // block back out so the workspace still builds.
    // let name = String::from("temp");
    // let moved = name;
    // println!("{name} {moved}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_count_handles_zero_and_nonzero() {
        assert_eq!(describe_count(0), "nothing here");
        assert_eq!(describe_count(3), "3 items");
    }

    #[test]
    fn find_first_admin_returns_the_first_match() {
        let users = [("alice", "member"), ("bo", "admin"), ("cy", "admin")];
        assert_eq!(find_first_admin(&users), Some("bo"));
    }
}

// TODO(reader): this crate's real lesson is its Cargo.toml. The code here
// just exists so the declared dependencies are used and the crate builds.

use serde::Serialize;

#[derive(Serialize)]
struct Ping {
    ok: bool,
}

fn main() {
    let payload = Ping { ok: true };
    println!("{}", serde_json::to_string(&payload).unwrap());

    // `once_cell::sync::Lazy` for a value computed once, on first access.
    static GREETING: once_cell::sync::Lazy<String> =
        once_cell::sync::Lazy::new(|| "hello from a Lazy static".to_string());
    println!("{}", *GREETING);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_serializes_to_expected_json() {
        let json = serde_json::to_string(&Ping { ok: true }).unwrap();
        assert_eq!(json, r#"{"ok":true}"#);
    }
}

// TODO(reader): `#[derive(...)]` generates code for you at compile time.
// As you read each derive, ask: what trait is this implementing, and what
// would I have to hand-write if it weren't here?

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    // `#[serde(default)]` fills this in with `Role::default()` if the
    // field is missing from the JSON, instead of failing to deserialize.
    #[serde(default)]
    role: Role,
    // `skip_serializing_if` omits the field from output entirely when the
    // condition holds, rather than emitting `"nickname":null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
enum Role {
    #[default]
    Member,
    Admin,
}

fn main() {
    let user = User {
        id: 1,
        name: "Priya".to_string(),
        role: Role::Admin,
        nickname: None,
    };

    // `Serialize` is what makes `serde_json::to_string` possible at all —
    // without it, this call wouldn't compile.
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("{json}");

    // `Deserialize` is what makes parsing back into a `User` possible.
    let round_tripped: User = serde_json::from_str(&json).unwrap();
    // `PartialEq` (also derived) is what makes `==` here compile at all.
    assert_eq!(user, round_tripped);
    println!("round-trip ok: {round_tripped:?}");

    // This JSON is missing "role" and "nickname" entirely. Without
    // `#[serde(default)]` on `role`, this would fail with a "missing
    // field" error instead of falling back to `Role::default()`.
    let minimal = r#"{"id": 2, "name": "Jae"}"#;
    let parsed: User = serde_json::from_str(minimal).unwrap();
    println!("parsed from minimal JSON: {parsed:?}");

    // `Clone` (derived) is what makes this line compile instead of
    // requiring a move or a manual field-by-field copy.
    let user_copy = user.clone();
    println!("clone: {user_copy:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_role_falls_back_to_default() {
        let parsed: User = serde_json::from_str(r#"{"id": 2, "name": "Jae"}"#).unwrap();
        assert_eq!(parsed.role, Role::Member);
    }

    #[test]
    fn nickname_is_omitted_from_json_when_none() {
        let user = User {
            id: 1,
            name: "A".into(),
            role: Role::Member,
            nickname: None,
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(!json.contains("nickname"));
    }
}

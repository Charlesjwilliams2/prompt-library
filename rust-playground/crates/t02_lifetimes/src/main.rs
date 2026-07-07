// TODO(reader): lifetimes don't change how long a value lives; they
// describe, to the compiler, how long references are *allowed* to be used.
// As you read, ask "what would dangle if this were allowed?"

// This struct can't exist without borrowing from someone else's `String`,
// so it needs a lifetime parameter tying its reference to that owner.
struct Excerpt<'a> {
    source_title: &'a str,
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, note: &str) -> String {
        format!("{note}: \"{}\" (from {})", self.text, self.source_title)
    }
}

// Elided lifetimes: the compiler infers a single lifetime here because
// there's only one reference input, so the output must borrow from it.
fn first_sentence(text: &str) -> &str {
    text.split('.').next().unwrap_or(text).trim()
}

// Explicit lifetime required: two reference inputs, and the compiler can't
// guess which one the output borrows from without being told.
fn pick_shorter<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() <= second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let article = String::from("Rust prevents dangling references. It does this at compile time.");

    let excerpt = Excerpt {
        source_title: "Rust Book, ch. 10",
        text: first_sentence(&article),
    };
    println!("{}", excerpt.announce("key point"));

    let a = String::from("a short one");
    let b = String::from("a noticeably longer sentence");
    println!("shorter: {}", pick_shorter(&a, &b));

    // TODO(reader): uncomment this block to see a real "borrowed value does
    // not live long enough" error, then comment it back out.
    // let dangling: &str;
    // {
    //     let temp = String::from("temporary");
    //     dangling = first_sentence(&temp);
    // }
    // println!("{dangling}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sentence_stops_at_period() {
        assert_eq!(first_sentence("One. Two."), "One");
    }

    #[test]
    fn pick_shorter_returns_the_shorter_str() {
        assert_eq!(pick_shorter("aaa", "a"), "a");
    }
}

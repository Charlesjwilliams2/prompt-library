// TODO(reader): Rust has two ways to say "any type that implements this
// trait": generics (`impl Trait` / `<T: Trait>`), resolved at compile time,
// and trait objects (`dyn Trait`), resolved at runtime through a vtable.
// As you read, ask which one each function chose, and whether it had to.

trait Notifier {
    fn notify(&self, message: &str) -> String;
}

struct EmailNotifier {
    address: String,
}

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) -> String {
        format!("email to {}: {message}", self.address)
    }
}

struct SmsNotifier {
    phone: String,
}

impl Notifier for SmsNotifier {
    fn notify(&self, message: &str) -> String {
        format!("sms to {}: {message}", self.phone)
    }
}

// Static dispatch: the compiler generates a separate copy of this function
// for every concrete `N` it's called with. No indirection at runtime, but
// the concrete type must be known at the call site.
fn notify_once<N: Notifier>(notifier: &N, message: &str) -> String {
    notifier.notify(message)
}

// Dynamic dispatch: one function, works with any `Notifier` behind a
// pointer, at the cost of a vtable lookup per call. This is what you need
// when the concrete types aren't known until runtime, like a mixed list.
fn notify_all(notifiers: &[Box<dyn Notifier>], message: &str) -> Vec<String> {
    notifiers.iter().map(|n| n.notify(message)).collect()
}

fn main() {
    let email = EmailNotifier {
        address: "ops@example.com".into(),
    };
    let sms = SmsNotifier {
        phone: "+1-555-0100".into(),
    };

    // Static dispatch: `notify_once::<EmailNotifier>` and
    // `notify_once::<SmsNotifier>` are two distinct compiled functions.
    println!("{}", notify_once(&email, "build finished"));
    println!("{}", notify_once(&sms, "build finished"));

    // Dynamic dispatch: a single `Vec` holding two different concrete
    // types, unified behind `Box<dyn Notifier>`.
    let mixed: Vec<Box<dyn Notifier>> = vec![Box::new(email), Box::new(sms)];
    for line in notify_all(&mixed, "deploy started") {
        println!("{line}");
    }

    // TODO(reader): could `notify_all` have been written as
    // `fn notify_all<N: Notifier>(notifiers: &[N], message: &str)` instead?
    // Try it mentally against the `mixed` vector above before checking
    // WORKBOOK.md.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notify_once_dispatches_statically() {
        let email = EmailNotifier {
            address: "a@b.com".into(),
        };
        assert_eq!(notify_once(&email, "hi"), "email to a@b.com: hi");
    }

    #[test]
    fn notify_all_dispatches_dynamically_over_mixed_types() {
        let notifiers: Vec<Box<dyn Notifier>> = vec![
            Box::new(EmailNotifier {
                address: "a@b.com".into(),
            }),
            Box::new(SmsNotifier {
                phone: "555".into(),
            }),
        ];
        let messages = notify_all(&notifiers, "hi");
        assert_eq!(messages, vec!["email to a@b.com: hi", "sms to 555: hi"]);
    }
}

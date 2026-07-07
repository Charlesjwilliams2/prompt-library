// TODO(reader): before reading WORKBOOK.md, read this file top to bottom
// and try to say out loud, for each variable, whether it is *owned*,
// *moved*, or *borrowed* at each point it's used.

#[derive(Debug)]
struct Report {
    title: String,
    lines: Vec<String>,
}

fn summarize(report: &Report) -> String {
    format!("{} ({} lines)", report.title, report.lines.len())
}

fn add_line(report: &mut Report, line: String) {
    report.lines.push(line);
}

// TODO(reader): this function takes ownership of `report` instead of
// borrowing it. Why might an LLM write it this way, and what does the
// caller lose by calling it?
fn archive(report: Report) -> String {
    format!("ARCHIVED: {}", report.title)
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let mut report = Report {
        title: String::from("Q3 Notes"),
        lines: Vec::new(),
    };

    add_line(&mut report, String::from("revenue up 12%"));
    add_line(&mut report, String::from("churn flat"));

    // `summarize` only borrows `report`, so we can keep using it afterward.
    println!("{}", summarize(&report));

    let title_copy: String = report.title.clone();
    let title_ref: &str = &report.title;
    println!("clone: {title_copy}, borrow: {title_ref}");

    let a = String::from("short");
    let b = String::from("a bit longer");
    println!("longest: {}", longest(&a, &b));

    // `archive` takes ownership, so `report` cannot be used after this line.
    let archived = archive(report);
    println!("{archived}");

    // Uncomment the next line to see the borrow checker reject a
    // use-after-move: `report` was moved into `archive` above.
    // println!("{}", summarize(&report));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_reports_line_count() {
        let report = Report {
            title: "T".into(),
            lines: vec!["a".into(), "b".into()],
        };
        assert_eq!(summarize(&report), "T (2 lines)");
    }

    #[test]
    fn longest_picks_the_longer_slice() {
        assert_eq!(longest("short", "a bit longer"), "a bit longer");
    }
}

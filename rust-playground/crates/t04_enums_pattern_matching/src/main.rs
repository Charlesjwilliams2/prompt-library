// TODO(reader): Rust enums can carry data per-variant, which is what makes
// `match` so different from a switch statement in most other languages.
// As you read, ask: could any state be represented that shouldn't be?

#[derive(Debug)]
enum JobStatus {
    Queued,
    Running { progress_percent: u8 },
    Failed { reason: String, retryable: bool },
    Completed { output_path: String },
}

fn describe(status: &JobStatus) -> String {
    match status {
        JobStatus::Queued => "waiting to start".to_string(),
        JobStatus::Running { progress_percent } if *progress_percent >= 100 => {
            "finishing up".to_string()
        }
        JobStatus::Running { progress_percent } => format!("running ({progress_percent}%)"),
        JobStatus::Failed {
            reason,
            retryable: true,
        } => {
            format!("failed (will retry): {reason}")
        }
        JobStatus::Failed {
            reason,
            retryable: false,
        } => {
            format!("failed (permanent): {reason}")
        }
        JobStatus::Completed { output_path } => format!("done -> {output_path}"),
    }
}

fn is_terminal(status: &JobStatus) -> bool {
    // `matches!` is a compact way to ask "is this one of these shapes?"
    // without binding the inner data.
    matches!(
        status,
        JobStatus::Completed { .. } | JobStatus::Failed { .. }
    )
}

fn main() {
    let jobs = vec![
        JobStatus::Queued,
        JobStatus::Running {
            progress_percent: 40,
        },
        JobStatus::Running {
            progress_percent: 100,
        },
        JobStatus::Failed {
            reason: "disk full".into(),
            retryable: true,
        },
        JobStatus::Failed {
            reason: "bad input".into(),
            retryable: false,
        },
        JobStatus::Completed {
            output_path: "/tmp/out.bin".into(),
        },
    ];

    for job in &jobs {
        println!(
            "{:>22} | terminal={} | {}",
            format!("{job:?}"),
            is_terminal(job),
            describe(job)
        );
    }

    // `if let` is for when you only care about one variant and want to
    // ignore the rest without writing a full `match`.
    if let JobStatus::Running { progress_percent } = &jobs[1] {
        println!("job 1 is at {progress_percent}%");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_at_full_progress_reads_as_finishing_up() {
        let status = JobStatus::Running {
            progress_percent: 100,
        };
        assert_eq!(describe(&status), "finishing up");
    }

    #[test]
    fn queued_is_not_terminal_but_completed_is() {
        assert!(!is_terminal(&JobStatus::Queued));
        assert!(is_terminal(&JobStatus::Completed {
            output_path: "x".into()
        }));
    }
}

# Inbox — Agentic Study Workflow

This is where you drop real, unmodified Rust snippets you run into
elsewhere — another repo, an agentic coding session, a PR someone sent
you, a Stack Overflow answer an LLM wrote. The `crates/` curriculum
teaches the checklist; the inbox is where you practice applying it to
code you didn't write for the purpose of teaching.

## How to add one

1. Make a new folder named `YYYY-MM-DD_short-description/`.
2. Paste the snippet in as `snippet.rs`, verbatim — don't clean it up
   first. The point is to practice reading it as-is.
3. Copy `_TEMPLATE.md` into the new folder as `analysis.md` and fill in
   the "Source/context" and "Which curriculum topics apply" sections
   yourself.
4. In a future Claude Code session, point at the folder and ask for help
   filling in "Claude's annotations" — reference the relevant
   `crates/NN_*/WORKBOOK.md` checklists so the annotation stays anchored
   to what you've already studied, not a generic code review.
5. Write your own "Verdict" once you and Claude have gone through it.

## Why dated folders, not one running log

Keeping each snippet in its own folder makes it easy to revisit an old
one later and see if your reading of it changes as you learn more —
compare your first-pass notes against how you'd read the same code six
months from now.

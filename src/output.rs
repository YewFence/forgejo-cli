use std::io::Write;

use comfy_table::presets::NOTHING;
use comfy_table::{ContentArrangement, Table};
use serde::Serialize;

/// Print a list of items as a table to stdout.
///
/// In `--json` mode, serializes `items` to JSON and prints that instead.
/// Empty lists print `[]` in JSON mode and an info message in human mode.
pub fn print_list<T: Serialize>(items: &[T], headers: &[&str], row_fn: impl Fn(&T) -> Vec<String>) {
    if crate::json_mode() {
        match serde_json::to_value(items) {
            Ok(val) => print_json(&val),
            Err(e) => error(&format!("Failed to serialize to JSON: {e}")),
        }
        return;
    }

    if items.is_empty() {
        info("No results");
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(NOTHING)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(terminal_width())
        .set_header(headers);

    for item in items {
        table.add_row(row_fn(item));
    }

    println!("{table}");
}

/// Print a single item as JSON, or run the human-format closure.
///
/// Use for view commands: `output::print_or_json(&item, || { /* human output */ })`
pub fn print_or_json<T: Serialize>(
    item: &T,
    human_fn: impl FnOnce() -> eyre::Result<()>,
) -> eyre::Result<()> {
    if crate::json_mode() {
        match serde_json::to_value(item) {
            Ok(val) => {
                print_json(&val);
                Ok(())
            }
            Err(e) => eyre::bail!("Failed to serialize to JSON: {e}"),
        }
    } else {
        human_fn()
    }
}

/// Print a JSON value to stdout (pretty-printed).
pub fn print_json(value: &serde_json::Value) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    // Safe: serde_json only fails on io errors or non-string map keys
    let _ = serde_json::to_writer_pretty(&mut handle, value);
    let _ = handle.write_all(b"\n");
}

/// Print a success message to stderr with a green checkmark.
pub fn success(msg: &str) {
    let crate::SpecialRender {
        bright_green,
        checkmark,
        reset,
        ..
    } = crate::special_render();
    eprintln!("{bright_green}{checkmark}{reset} {msg}");
}

/// Print an error/failure message to stderr with a red cross.
pub fn error(msg: &str) {
    let crate::SpecialRender {
        bright_red,
        cross,
        reset,
        ..
    } = crate::special_render();
    eprintln!("{bright_red}{cross}{reset} {msg}");
}

/// Print a dimmed informational message to stderr.
pub fn info(msg: &str) {
    let crate::SpecialRender {
        dark_grey, reset, ..
    } = crate::special_render();
    eprintln!("{dark_grey}{msg}{reset}");
}

/// Print a verbose message to stderr.
///
/// Callers are responsible for gating on `verbose_mode()`.
/// The `verbose_log!` macro does this automatically.
pub fn verbose(msg: &str) {
    let crate::SpecialRender {
        dark_grey, reset, ..
    } = crate::special_render();
    eprintln!("{dark_grey}[verbose] {msg}{reset}");
}

/// Print a dry-run preview message to stderr.
pub fn dry_run(msg: &str) {
    let crate::SpecialRender { yellow, reset, .. } = crate::special_render();
    eprintln!("{yellow}[dry-run]{reset} Would {msg}");
}

/// Format a relative time string from an OffsetDateTime (e.g., "2h", "3d").
pub fn relative_time(dt: &time::OffsetDateTime) -> String {
    let now = time::OffsetDateTime::now_utc();
    let duration = now - *dt;

    let seconds = duration.whole_seconds();
    if seconds < 0 {
        return "future".to_string();
    }

    let minutes = duration.whole_minutes();
    let hours = duration.whole_hours();
    let days = duration.whole_days();
    let weeks = days / 7;
    let months = days / 30;
    let years = days / 365;

    if seconds < 60 {
        "now".to_string()
    } else if minutes < 60 {
        format!("{minutes}m")
    } else if hours < 24 {
        format!("{hours}h")
    } else if days < 7 {
        format!("{days}d")
    } else if weeks < 5 {
        format!("{weeks}w")
    } else if years < 1 {
        format!("{months}mo")
    } else {
        format!("{years}y")
    }
}

/// Get the terminal width, falling back to 80.
fn terminal_width() -> u16 {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0)
        .unwrap_or(80)
}

/// Color a state string (green for open, red for closed).
pub fn colored_state(state: &forgejo_api::structs::StateType) -> String {
    let crate::SpecialRender {
        bright_green,
        bright_red,
        reset,
        ..
    } = crate::special_render();
    match state {
        forgejo_api::structs::StateType::Open => format!("{bright_green}open{reset}"),
        forgejo_api::structs::StateType::Closed => format!("{bright_red}closed{reset}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;
    use time::OffsetDateTime;

    fn ago(duration: Duration) -> OffsetDateTime {
        OffsetDateTime::now_utc() - duration
    }

    #[test]
    fn relative_time_just_now() {
        let dt = ago(Duration::seconds(30));
        assert_eq!(relative_time(&dt), "now");
    }

    #[test]
    fn relative_time_minutes() {
        let dt = ago(Duration::minutes(5));
        assert_eq!(relative_time(&dt), "5m");
    }

    #[test]
    fn relative_time_hours() {
        let dt = ago(Duration::hours(3));
        assert_eq!(relative_time(&dt), "3h");
    }

    #[test]
    fn relative_time_days() {
        let dt = ago(Duration::days(4));
        assert_eq!(relative_time(&dt), "4d");
    }

    #[test]
    fn relative_time_weeks() {
        let dt = ago(Duration::weeks(2));
        assert_eq!(relative_time(&dt), "2w");
    }

    #[test]
    fn relative_time_months() {
        let dt = ago(Duration::days(90));
        assert_eq!(relative_time(&dt), "3mo");
    }

    #[test]
    fn relative_time_years() {
        let dt = ago(Duration::days(400));
        assert_eq!(relative_time(&dt), "1y");
    }

    #[test]
    fn relative_time_future() {
        let dt = OffsetDateTime::now_utc() + Duration::hours(1);
        assert_eq!(relative_time(&dt), "future");
    }
}

use std::{
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ratatui::{
    style::Stylize,
    text::{Line, Span},
};

use crate::{model::Task, runner::RunStatus, tui::components::relative_dir};

const DOTS_INTERVAL: Duration = Duration::from_millis(80);
const DOTS_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub(crate) fn run_line(status: RunStatus, task: &Task, root: &Path) -> Line<'static> {
    Line::from(vec![
        indicator(status),
        format!(
            " ({}) {}",
            relative_dir(root, &task.cwd).display(),
            task.command_line()
        )
        .into(),
    ])
}

fn indicator(status: RunStatus) -> Span<'static> {
    match status {
        RunStatus::Running => dots_frame(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
        )
        .blue(),
        RunStatus::Succeeded => "o".green(),
        RunStatus::Failed => "!".red(),
    }
}

fn dots_frame(elapsed: Duration) -> &'static str {
    DOTS_FRAMES[(elapsed.as_millis() / DOTS_INTERVAL.as_millis()) as usize % DOTS_FRAMES.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dots_spinner_uses_cli_spinners_frames_and_interval() {
        for (index, frame) in DOTS_FRAMES.iter().enumerate() {
            assert_eq!(dots_frame(DOTS_INTERVAL * index as u32), *frame);
        }
        assert_eq!(
            dots_frame(DOTS_INTERVAL * DOTS_FRAMES.len() as u32),
            DOTS_FRAMES[0]
        );
    }
}

mod app;
mod model;
mod providers;
mod runner;
mod scan;

use std::process::{Command, ExitCode};

fn main() -> color_eyre::Result<ExitCode> {
    color_eyre::install()?;
    let root = std::env::current_dir()?;
    let workspaces = scan::scan(root.clone(), 3);
    let selected = ratatui::run(|terminal| app::App::new(root, workspaces).run(terminal))?;

    let Some(task) = selected else {
        return Ok(ExitCode::SUCCESS);
    };
    let status = Command::new(&task.command)
        .args(&task.args)
        .current_dir(&task.cwd)
        .status()?;
    Ok(ExitCode::from(status.code().unwrap_or(1) as u8))
}

mod ui;

use ratatui::DefaultTerminal;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    ui::app::App::default().run(terminal)
}

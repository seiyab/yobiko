use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph},
};

pub(crate) fn render_help(frame: &mut Frame, area: Rect) {
    let popup = centered(area, 80, 80);
    let help = [
        ("q", "exit from yobiko"),
        ("? / Esc", "toggle help"),
        ("j/k", "move cursor down/up"),
        ("Ctrl-n", "move cursor down"),
        ("/", "input search query"),
        ("Enter", "launch task under the cursor"),
        ("Ctrl-d", "switch to dashboard"),
        ("L/P/H", "open launcher/project/history view"),
        ("r", "rerun selected history task"),
        ("x", "kill selected history task"),
    ];
    let lines = help
        .into_iter()
        .map(|(key, description)| Line::from(vec![format!("{key:8}: ").bold(), description.into()]))
        .collect::<Vec<_>>();
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(Block::bordered().title(" [?]Help "))
            .alignment(Alignment::Left),
        popup,
    );
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ],
    )
    .split(area);
    Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ],
    )
    .split(vertical[1])[1]
}

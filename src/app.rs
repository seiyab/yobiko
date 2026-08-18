use std::time::{Duration, SystemTime, UNIX_EPOCH};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};

use crate::{
    model::{Task, Workspace},
    runner::{RunStatus, Runner},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    OneShot,
    Dashboard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Launcher,
    Project,
    History,
}

pub struct App {
    workspaces: Vec<Workspace>,
    tasks: Vec<Task>,
    runner: Runner,
    mode: Mode,
    tab: Tab,
    selected: usize,
    history_selected: usize,
    query: String,
    searching: bool,
    help: bool,
    exit: bool,
    one_shot: Option<Task>,
    error: Option<String>,
}

impl App {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        let tasks = workspaces
            .iter()
            .flat_map(|workspace| workspace.tasks.clone())
            .collect();
        Self {
            workspaces,
            tasks,
            runner: Runner::default(),
            mode: Mode::OneShot,
            tab: Tab::Launcher,
            selected: 0,
            history_selected: 0,
            query: String::new(),
            searching: false,
            help: false,
            exit: false,
            one_shot: None,
            error: None,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Task>> {
        while !self.exit {
            self.runner.refresh();
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.handle_key(key);
            }
        }
        Ok(self.one_shot)
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.help {
            if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
                self.help = false;
            }
            return;
        }
        if self.searching {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => self.searching = false,
                KeyCode::Backspace => {
                    self.query.pop();
                    self.selected = 0;
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.query.clear();
                    self.selected = 0;
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.query.push(c);
                    self.selected = 0;
                }
                _ => {}
            }
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('?') => self.help = true,
            KeyCode::Char('d')
                if key.modifiers.contains(KeyModifiers::CONTROL) && self.mode == Mode::OneShot =>
            {
                self.mode = Mode::Dashboard
            }
            KeyCode::Char('L') if self.mode == Mode::Dashboard => self.tab = Tab::Launcher,
            KeyCode::Char('P') if self.mode == Mode::Dashboard => self.tab = Tab::Project,
            KeyCode::Char('H') if self.mode == Mode::Dashboard => self.tab = Tab::History,
            KeyCode::Char('/') if self.tab == Tab::Launcher || self.mode == Mode::OneShot => {
                self.searching = true
            }
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Enter => self.launch_selected(),
            KeyCode::Char('r') if self.tab == Tab::History => self.rerun_selected(),
            KeyCode::Char('x') if self.tab == Tab::History => self.kill_selected(),
            _ => {}
        }
    }

    fn move_down(&mut self) {
        if self.mode == Mode::Dashboard && self.tab == Tab::History {
            self.history_selected =
                (self.history_selected + 1).min(self.runner.runs.len().saturating_sub(1));
        } else {
            self.selected = (self.selected + 1).min(self.filtered_tasks().len().saturating_sub(1));
        }
    }

    fn move_up(&mut self) {
        if self.mode == Mode::Dashboard && self.tab == Tab::History {
            self.history_selected = self.history_selected.saturating_sub(1);
        } else {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    fn selected_task(&self) -> Option<Task> {
        self.filtered_tasks().get(self.selected).copied().cloned()
    }

    fn filtered_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| {
                self.query.is_empty()
                    || task.name.contains(&self.query)
                    || task.cwd.to_string_lossy().contains(&self.query)
                    || task.command.contains(&self.query)
            })
            .collect()
    }

    fn launch_selected(&mut self) {
        let Some(task) = self.selected_task() else {
            return;
        };
        if self.mode == Mode::OneShot {
            self.one_shot = Some(task);
            self.exit = true;
        } else if self.tab == Tab::Launcher {
            self.spawn(task);
        }
    }

    fn history_index(&self) -> Option<usize> {
        self.runner
            .runs
            .len()
            .checked_sub(self.history_selected + 1)
    }
    fn rerun_selected(&mut self) {
        if let Some(task) = self
            .history_index()
            .and_then(|i| self.runner.runs.get(i))
            .map(|r| r.task.clone())
        {
            self.spawn(task);
        }
    }
    fn kill_selected(&mut self) {
        if let Some(index) = self.history_index() {
            self.runner.kill(index);
        }
    }
    fn spawn(&mut self, task: Task) {
        if let Err(error) = self.runner.spawn(task) {
            self.error = Some(error.to_string());
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let rows = if self.mode == Mode::Dashboard {
            vec![
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
        } else {
            vec![Constraint::Min(1), Constraint::Length(1)]
        };
        let chunks = Layout::vertical(rows).split(area);
        let body = if self.mode == Mode::Dashboard {
            let titles = ["[L]auncher", "[P]roject", "[H]istory"];
            let selected = match self.tab {
                Tab::Launcher => 0,
                Tab::Project => 1,
                Tab::History => 2,
            };
            frame.render_widget(
                Tabs::new(titles)
                    .select(selected)
                    .divider("   ")
                    .highlight_style(Modifier::UNDERLINED),
                chunks[0],
            );
            chunks[1]
        } else {
            chunks[0]
        };
        match (self.mode, self.tab) {
            (Mode::OneShot, _) | (Mode::Dashboard, Tab::Launcher) => {
                self.draw_launcher(frame, body)
            }
            (Mode::Dashboard, Tab::Project) => self.draw_projects(frame, body),
            (Mode::Dashboard, Tab::History) => self.draw_history(frame, body),
        }
        let footer = *chunks.last().expect("layout has a footer");
        let status = self
            .error
            .as_deref()
            .map_or_else(|| "Yobiko  [?] help".into(), |e| format!("Error: {e}"));
        frame.render_widget(Paragraph::new(status), footer);
        if self.help {
            self.draw_help(frame, area);
        }
    }

    fn draw_launcher(&self, frame: &mut Frame, area: Rect) {
        let rows =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
        let columns = Layout::horizontal([
            Constraint::Percentage(if self.mode == Mode::Dashboard { 50 } else { 0 }),
            Constraint::Percentage(if self.mode == Mode::Dashboard {
                50
            } else {
                100
            }),
        ])
        .split(rows[1]);
        self.draw_tasks(frame, rows[0]);
        if self.mode == Mode::Dashboard {
            self.draw_runs(frame, columns[0]);
        }
        self.draw_detail(frame, columns[1]);
    }

    fn draw_tasks(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(" Select Task ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let rows = Layout::vertical([Constraint::Length(2), Constraint::Min(1)]).split(inner);
        let cursor = if self.searching { "_" } else { "" };
        frame.render_widget(
            Paragraph::new(format!("Filter[/]: {}{cursor}", self.query)).block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_style(Color::DarkGray),
            ),
            rows[0],
        );
        let tasks = self.filtered_tasks();
        let items = tasks
            .iter()
            .map(|task| ListItem::new(format!("({}) {}", task.cwd.display(), task.command_line())))
            .collect::<Vec<_>>();
        let mut state = ListState::default().with_selected(
            (!items.is_empty()).then_some(self.selected.min(items.len().saturating_sub(1))),
        );
        frame.render_stateful_widget(
            List::new(items)
                .highlight_symbol("> ")
                .highlight_style(Style::new().add_modifier(Modifier::BOLD)),
            rows[1],
            &mut state,
        );
    }

    fn draw_detail(&self, frame: &mut Frame, area: Rect) {
        let text = self.selected_task().map_or_else(String::new, |task| {
            format!(
                "directory: {}\nname: {}\ncommand: {}{}",
                task.cwd.display(),
                task.name,
                task.command_line(),
                task.content
                    .map_or_else(String::new, |c| format!("\ncontent: {c}"))
            )
        });
        frame.render_widget(
            Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .block(Block::bordered().title(" Detail ")),
            area,
        );
    }

    fn draw_runs(&self, frame: &mut Frame, area: Rect) {
        let items = self
            .runner
            .runs
            .iter()
            .rev()
            .map(|run| ListItem::new(run_line(run.status, &run.task)))
            .collect::<Vec<_>>();
        frame.render_widget(
            List::new(items).block(Block::bordered().title(" Runs ")),
            area,
        );
    }

    fn draw_history(&self, frame: &mut Frame, area: Rect) {
        let columns = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        let items = self
            .runner
            .runs
            .iter()
            .rev()
            .map(|run| ListItem::new(run_line(run.status, &run.task)))
            .collect::<Vec<_>>();
        let mut state = ListState::default().with_selected(
            (!items.is_empty()).then_some(self.history_selected.min(items.len().saturating_sub(1))),
        );
        frame.render_stateful_widget(
            List::new(items)
                .block(Block::bordered().title(" Runs "))
                .highlight_symbol("> ")
                .highlight_style(Modifier::BOLD),
            columns[0],
            &mut state,
        );
        let output = self
            .history_index()
            .and_then(|i| self.runner.runs.get(i))
            .map(|r| r.output.as_str())
            .unwrap_or("");
        frame.render_widget(
            Paragraph::new(output)
                .wrap(Wrap { trim: false })
                .block(Block::bordered().title(" Output ")),
            columns[1],
        );
    }

    fn draw_projects(&self, frame: &mut Frame, area: Rect) {
        let items = self
            .workspaces
            .iter()
            .map(|w| {
                ListItem::new(format!(
                    "{} ({}, {} tasks)",
                    w.dir.display(),
                    w.provider,
                    w.tasks.len()
                ))
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            List::new(items).block(Block::bordered().title(" Projects ")),
            area,
        );
    }

    fn draw_help(&self, frame: &mut Frame, area: Rect) {
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
            .map(|(key, description)| {
                Line::from(vec![format!("{key:8}: ").bold(), description.into()])
            })
            .collect::<Vec<_>>();
        frame.render_widget(Clear, popup);
        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .block(Block::bordered().title(" [?]Help "))
                .alignment(Alignment::Left),
            popup,
        );
    }
}

const DOTS_INTERVAL: Duration = Duration::from_millis(80);
const DOTS_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn run_line(status: RunStatus, task: &Task) -> Line<'static> {
    Line::from(vec![
        indicator(status),
        format!(" ({}) {}", task.cwd.display(), task.command_line()).into(),
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

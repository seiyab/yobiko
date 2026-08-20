use std::{path::PathBuf, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};

use crate::{
    model::{Task, Workspace},
    runner::Runner,
    tui::components::{relative_dir, render_help, render_task_detail, run_line},
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
    root: PathBuf,
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
    pub fn new(root: PathBuf, workspaces: Vec<Workspace>) -> Self {
        let tasks = workspaces
            .iter()
            .flat_map(|workspace| workspace.tasks.clone())
            .collect();
        Self {
            root,
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
            render_help(frame, area);
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
            .map(|task| {
                ListItem::new(Line::from(vec![
                    format!("({})", relative_dir(&self.root, &task.cwd).display()).dark_gray(),
                    " ".into(),
                    task.command_line().into(),
                ]))
            })
            .collect::<Vec<_>>();
        let mut state = ListState::default().with_selected(
            (!items.is_empty()).then_some(self.selected.min(items.len().saturating_sub(1))),
        );
        frame.render_stateful_widget(
            List::new(items)
                .highlight_symbol("> ")
                .highlight_style(Style::new().add_modifier(Modifier::BOLD))
                .scroll_padding(2),
            rows[1],
            &mut state,
        );
    }

    fn draw_detail(&self, frame: &mut Frame, area: Rect) {
        render_task_detail(frame, area, &self.root, self.selected_task().as_ref());
    }

    fn draw_runs(&self, frame: &mut Frame, area: Rect) {
        let items = self
            .runner
            .runs
            .iter()
            .rev()
            .map(|run| ListItem::new(run_line(run.status, &run.task, &self.root)))
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
            .map(|run| ListItem::new(run_line(run.status, &run.task, &self.root)))
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
                    relative_dir(&self.root, &w.dir).display(),
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
}

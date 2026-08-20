use std::{path::PathBuf, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Block, List, ListItem, Paragraph, Tabs},
};

use crate::{
    model::{Task, Workspace},
    runner::Runner,
    tui::components::{TaskPicker, relative_dir, render_help, render_task_detail, run_line},
    tui::screens::{History, HistoryEvent},
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
    task_picker: TaskPicker,
    history: History,
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
            task_picker: TaskPicker::default(),
            history: History::default(),
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
        let task_picker_active = self.mode == Mode::OneShot || self.tab == Tab::Launcher;
        if task_picker_active && self.task_picker.is_searching() {
            self.task_picker.handle_key(key, &self.tasks);
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
            _ if task_picker_active => {
                if let Some(task) = self.task_picker.handle_key(key, &self.tasks) {
                    self.launch(task);
                }
            }
            _ if self.mode == Mode::Dashboard && self.tab == Tab::History => {
                match self.history.handle_key(key, &self.runner) {
                    Some(HistoryEvent::Rerun(task)) => self.spawn(task),
                    Some(HistoryEvent::Kill(index)) => self.runner.kill(index),
                    None => {}
                }
            }
            _ => {}
        }
    }

    fn selected_task(&self) -> Option<Task> {
        self.task_picker.selected_task(&self.tasks).cloned()
    }

    fn launch(&mut self, task: Task) {
        if self.mode == Mode::OneShot {
            self.one_shot = Some(task);
            self.exit = true;
        } else if self.tab == Tab::Launcher {
            self.spawn(task);
        }
    }

    fn spawn(&mut self, task: Task) {
        if let Err(error) = self.runner.spawn(task) {
            self.error = Some(error.to_string());
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
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
            (Mode::Dashboard, Tab::History) => {
                self.history.render(frame, body, &self.root, &self.runner)
            }
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

    fn draw_launcher(&mut self, frame: &mut Frame, area: Rect) {
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
        self.task_picker
            .render(frame, rows[0], &self.root, &self.tasks);
        if self.mode == Mode::Dashboard {
            self.draw_runs(frame, columns[0]);
        }
        self.draw_detail(frame, columns[1]);
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

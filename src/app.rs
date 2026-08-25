use std::{path::PathBuf, time::Duration};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::Paragraph,
};

use crate::{
    model::{Task, Workspace},
    runner::Runner,
    tui::{
        Action,
        components::{CommandEditor, EditorResult, LaunchMode, render_help},
        screens::{Dashboard, OneShot},
    },
};

enum Screen {
    OneShot(OneShot),
    Dashboard(Dashboard),
}

pub struct App {
    root: PathBuf,
    workspaces: Vec<Workspace>,
    tasks: Vec<Task>,
    runner: Runner,
    screen: Screen,
    help: bool,
    exit: bool,
    one_shot: Option<Task>,
    error: Option<String>,
    command_editor: Option<CommandEditor>,
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
            screen: Screen::OneShot(OneShot::default()),
            help: false,
            exit: false,
            one_shot: None,
            error: None,
            command_editor: None,
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
        if self.command_editor.is_some() {
            self.handle_editor_key(key);
            return;
        }
        if self.help {
            if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
                self.help = false;
            }
            return;
        }
        if self.screen_captures_input() {
            self.handle_screen_key(key);
            return;
        }
        let action = match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ToggleHelp),
            KeyCode::Char('d')
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && matches!(self.screen, Screen::OneShot(_)) =>
            {
                Some(Action::OpenDashboard)
            }
            _ => self.screen_action(key),
        };
        if let Some(action) = action {
            self.apply(action);
        }
    }

    fn screen_captures_input(&self) -> bool {
        match &self.screen {
            Screen::OneShot(screen) => screen.captures_input(),
            Screen::Dashboard(screen) => screen.captures_input(),
        }
    }

    fn handle_screen_key(&mut self, key: KeyEvent) {
        if let Some(action) = self.screen_action(key) {
            self.apply(action);
        }
    }

    fn screen_action(&mut self, key: KeyEvent) -> Option<Action> {
        match &mut self.screen {
            Screen::OneShot(screen) => screen.handle_key(key, &self.tasks),
            Screen::Dashboard(screen) => screen.handle_key(key, &self.tasks, &self.runner),
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::ToggleHelp => self.help = !self.help,
            Action::OpenDashboard => self.screen = Screen::Dashboard(Dashboard::default()),
            Action::Run(task) => self.spawn(task),
            Action::SelectOneShot(task) => {
                self.one_shot = Some(task);
                self.exit = true;
            }
            Action::Edit(task, mode) => {
                self.command_editor = Some(CommandEditor::new(task, mode));
            }
            Action::Kill(index) => self.runner.kill(index),
        }
    }

    fn handle_editor_key(&mut self, key: KeyEvent) {
        let result = self.command_editor.as_mut().unwrap().handle_key(key);
        match result {
            EditorResult::Continue => {}
            EditorResult::Cancel => self.command_editor = None,
            EditorResult::Launch(task, mode) => {
                self.command_editor = None;
                match mode {
                    LaunchMode::OneShot => {
                        self.one_shot = Some(task);
                        self.exit = true;
                    }
                    LaunchMode::Dashboard => self.spawn(task),
                }
            }
        }
    }

    fn spawn(&mut self, task: Task) {
        if let Err(error) = self.runner.spawn(task) {
            self.error = Some(error.to_string());
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let rows = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);
        match &mut self.screen {
            Screen::OneShot(screen) => screen.render(frame, rows[0], &self.root, &self.tasks),
            Screen::Dashboard(screen) => screen.render(
                frame,
                rows[0],
                &self.root,
                &self.workspaces,
                &self.tasks,
                &self.runner,
            ),
        }
        let status = self.error.as_deref().map_or_else(
            || "Yobiko  [?] help".into(),
            |error| format!("Error: {error}"),
        );
        frame.render_widget(Paragraph::new(status), rows[1]);
        if self.help {
            render_help(frame, area);
        }
        if let Some(editor) = &self.command_editor {
            editor.render(frame, area);
        }
    }
}

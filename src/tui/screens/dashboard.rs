use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::Tabs,
};

use crate::{
    model::{Task, Workspace},
    runner::Runner,
    tui::{
        Action,
        screens::{history::History, launcher::Launcher, projects::Projects},
    },
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Launcher,
    Projects,
    History,
}

#[derive(Default)]
pub(crate) struct Dashboard {
    tab: Tab,
    launcher: Launcher,
    projects: Projects,
    history: History,
}

impl Dashboard {
    pub(crate) fn captures_input(&self) -> bool {
        self.tab == Tab::Launcher && self.launcher.captures_input()
    }

    pub(crate) fn handle_key(
        &mut self,
        key: KeyEvent,
        root: &Path,
        tasks: &[Task],
        runner: &Runner,
    ) -> Option<Action> {
        match key.code {
            KeyCode::Char('L') => self.tab = Tab::Launcher,
            KeyCode::Char('P') => self.tab = Tab::Projects,
            KeyCode::Char('H') => self.tab = Tab::History,
            _ => {
                return match self.tab {
                    Tab::Launcher => self.launcher.handle_key(key, root, tasks),
                    Tab::Projects => None,
                    Tab::History => self.history.handle_key(key, runner),
                };
            }
        }
        None
    }

    pub(crate) fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        root: &Path,
        workspaces: &[Workspace],
        tasks: &[Task],
        runner: &Runner,
    ) {
        let rows = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
        let selected = match self.tab {
            Tab::Launcher => 0,
            Tab::Projects => 1,
            Tab::History => 2,
        };
        frame.render_widget(
            Tabs::new(["[L]auncher", "[P]roject", "[H]istory"])
                .select(selected)
                .divider("   ")
                .highlight_style(Modifier::UNDERLINED),
            rows[0],
        );
        match self.tab {
            Tab::Launcher => self.launcher.render(frame, rows[1], root, tasks, runner),
            Tab::Projects => self.projects.render(frame, rows[1], root, workspaces),
            Tab::History => self.history.render(frame, rows[1], root, runner),
        }
    }
}

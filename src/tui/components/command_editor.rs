use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Wrap},
};

use crate::model::Task;

#[derive(Clone, Copy)]
pub(crate) enum LaunchMode {
    OneShot,
    Dashboard,
}

pub(crate) enum EditorResult {
    Continue,
    Cancel,
    Launch(Task, LaunchMode),
}

pub(crate) struct CommandEditor {
    task: Task,
    mode: LaunchMode,
    input: Vec<char>,
    cursor: usize,
    error: Option<&'static str>,
}

impl CommandEditor {
    pub(crate) fn new(task: Task, mode: LaunchMode) -> Self {
        let input = task.command_line().chars().collect::<Vec<_>>();
        let cursor = input.len();
        Self {
            task,
            mode,
            input,
            cursor,
            error: None,
        }
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> EditorResult {
        self.error = None;
        match key.code {
            KeyCode::Esc => return EditorResult::Cancel,
            KeyCode::Enter => {
                let command = self.input.iter().collect::<String>();
                if command.trim().is_empty() {
                    self.error = Some("Command cannot be empty");
                } else {
                    return EditorResult::Launch(
                        self.task.clone().with_shell_command(command),
                        self.mode,
                    );
                }
            }
            KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Right => self.cursor = (self.cursor + 1).min(self.input.len()),
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = self.input.len(),
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.input.remove(self.cursor);
            }
            KeyCode::Delete if self.cursor < self.input.len() => {
                self.input.remove(self.cursor);
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.input.clear();
                self.cursor = 0;
            }
            KeyCode::Char(character)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.input.insert(self.cursor, character);
                self.cursor += 1;
            }
            _ => {}
        }
        EditorResult::Continue
    }

    pub(crate) fn render(&self, frame: &mut Frame, area: Rect) {
        let popup = centered(area, 80, 7);
        let before = self.input[..self.cursor].iter().collect::<String>();
        let cursor = self.input.get(self.cursor).copied().unwrap_or(' ');
        let after_start = if self.cursor < self.input.len() {
            self.cursor + 1
        } else {
            self.cursor
        };
        let after = self.input[after_start..].iter().collect::<String>();
        let command = Line::from(vec![
            Span::styled("sh -c  ", Style::new().add_modifier(Modifier::DIM)),
            Span::raw(before),
            Span::styled(
                cursor.to_string(),
                Style::new().add_modifier(Modifier::REVERSED),
            ),
            Span::raw(after),
        ]);
        let mut lines = vec![
            command,
            Line::raw(""),
            Line::raw("Enter: run   Esc: cancel"),
        ];
        if let Some(error) = self.error {
            lines.push(Line::styled(
                error,
                Style::new().add_modifier(Modifier::BOLD),
            ));
        }
        frame.render_widget(Clear, popup);
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .block(Block::bordered().title(" Edit Shell Command ")),
            popup,
        );
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Length(height.min(area.height)),
            Constraint::Fill(1),
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
    use std::path::PathBuf;

    use crossterm::event::{KeyEvent, KeyModifiers};

    use super::*;

    fn editor() -> CommandEditor {
        CommandEditor::new(
            Task {
                name: "test".into(),
                cwd: PathBuf::new(),
                command: "yarn".into(),
                args: vec!["run".into(), "test".into()],
                content: None,
            },
            LaunchMode::OneShot,
        )
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn appends_text_and_launches_through_sh() {
        let mut editor = editor();
        for character in " --runInBand".chars() {
            assert!(matches!(
                editor.handle_key(key(KeyCode::Char(character))),
                EditorResult::Continue
            ));
        }
        let EditorResult::Launch(task, LaunchMode::OneShot) =
            editor.handle_key(key(KeyCode::Enter))
        else {
            panic!("expected one-shot launch");
        };
        assert_eq!(task.command, "sh");
        assert_eq!(task.args, ["-c", "yarn run test --runInBand"]);
        assert_eq!(task.name, "test");
    }

    #[test]
    fn rejects_an_empty_command() {
        let mut editor = editor();
        editor.input.clear();
        editor.cursor = 0;
        assert!(matches!(
            editor.handle_key(key(KeyCode::Enter)),
            EditorResult::Continue
        ));
        assert_eq!(editor.error, Some("Command cannot be empty"));
    }

    #[test]
    fn render_supports_cursor_at_end() {
        let backend = ratatui::backend::TestBackend::new(80, 24);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| editor().render(frame, frame.area()))
            .unwrap();
    }
}

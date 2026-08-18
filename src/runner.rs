use std::{
    io::Read,
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

use color_eyre::Result;

use crate::model::Task;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Succeeded,
    Failed,
}

pub struct Run {
    pub task: Task,
    pub status: RunStatus,
    pub output: String,
    child: Option<Child>,
    receiver: Receiver<Vec<u8>>,
}

#[derive(Default)]
pub struct Runner {
    pub runs: Vec<Run>,
}

impl Runner {
    pub fn spawn(&mut self, task: Task) -> Result<()> {
        let mut child = Command::new(&task.command)
            .args(&task.args)
            .current_dir(&task.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let (sender, receiver) = mpsc::channel();
        for mut stream in [
            child
                .stdout
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
            child
                .stderr
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
        ]
        .into_iter()
        .flatten()
        {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut bytes = [0; 4096];
                while let Ok(count) = stream.read(&mut bytes) {
                    if count == 0 {
                        break;
                    }
                    if sender.send(bytes[..count].to_vec()).is_err() {
                        break;
                    }
                }
            });
        }
        self.runs.push(Run {
            task,
            status: RunStatus::Running,
            output: String::new(),
            child: Some(child),
            receiver,
        });
        Ok(())
    }

    pub fn refresh(&mut self) {
        for run in &mut self.runs {
            for bytes in run.receiver.try_iter() {
                run.output.push_str(&String::from_utf8_lossy(&bytes));
            }
            if let Some(child) = &mut run.child
                && let Ok(Some(status)) = child.try_wait()
            {
                run.status = if status.success() {
                    RunStatus::Succeeded
                } else {
                    RunStatus::Failed
                };
                run.child = None;
            }
        }
    }

    pub fn kill(&mut self, index: usize) {
        if let Some(child) = self.runs.get_mut(index).and_then(|run| run.child.as_mut()) {
            let _ = child.kill();
        }
    }
}

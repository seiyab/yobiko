use ratatui::crossterm::event::Event as CrossTermEvent;
use std::{
    os::unix::thread,
    sync::mpsc,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyEvent};

#[derive(Clone, Copy, Debug)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let tick_rate = Duration::from_millis(tick_rate_ms);
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrossTermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(AppEvent::Key(e))
                                } else {
                                    Ok(())
                                }
                            }
                            CrossTermEvent::Resize(w, h) => sender.send(AppEvent::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(AppEvent::Tick)
                            .expect("fail to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        return Self {
            sender,
            receiver,
            handler,
        };
    }

    pub fn next(&self) -> Result<AppEvent> {
        Ok(self.receiver.recv()?)
    }
}

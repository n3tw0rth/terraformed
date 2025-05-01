use crate::{
    event::{AppEvent, Event, EventHandler},
    panes::Pane,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use strum::{EnumCount, IntoEnumIterator};

/// Application State.
#[derive(Debug)]
pub struct App {
    /// Application running state.
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Active pane.
    pub active_pane: Pane,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            active_pane: Pane::Status,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs the application.
    ///
    /// This function is responsible for rendering the widget and handling events.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            match self.events.next().await? {
                Event::Tick => self.tick(),

                // Crossterm Events (e.g. key events)
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?;
                    }
                }

                // Application Events
                Event::App(app_event) => match app_event {
                    AppEvent::SetPane(pane) => self.set_pane(pane),
                    AppEvent::NextPane => self.next_pane(),
                    AppEvent::PreviousPane => self.previous_pane(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }

        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            // Quit
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            // Pane Traversal
            KeyCode::Right => self.events.send(AppEvent::NextPane),
            KeyCode::Left => self.events.send(AppEvent::PreviousPane),
            KeyCode::Char('1') => self.events.send(AppEvent::SetPane(Pane::Status)),
            KeyCode::Char('2') => self.events.send(AppEvent::SetPane(Pane::Runs)),
            KeyCode::Char('3') => self.events.send(AppEvent::SetPane(Pane::Workspaces)),

            _ => {}
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
        tracing::debug!("quit");
    }

    pub fn set_pane(&mut self, pane: Pane) {
        self.active_pane = pane;
        tracing::debug!(?self.active_pane, "set_pane");
    }

    pub fn next_pane(&mut self) {
        let idx = self.active_pane as usize;
        let next_idx = (idx + 1) % Pane::COUNT;

        if let Some(pane) = Pane::iter().nth(next_idx) {
            self.active_pane = pane;
            tracing::debug!(?self.active_pane, "next_pane");
        } else {
            tracing::warn!("next_pane: failed to get pane at index {}", next_idx);
        }
    }

    pub fn previous_pane(&mut self) {
        let idx = self.active_pane as usize;
        let prev_idx = (idx + Pane::COUNT - 1) % Pane::COUNT;

        if let Some(pane) = Pane::iter().nth(prev_idx) {
            self.active_pane = pane;
            tracing::debug!(?self.active_pane, "previous_pane");
        } else {
            tracing::warn!("previous_pane: failed to get pane at index {}", prev_idx);
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}
}

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::panes::workspaces::WorkspacesPane;
use crate::{
    app::App,
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [body, footer] = Layout::vertical([
            Constraint::Min(0), // Takes whatever space is left
            Constraint::Length(3),
        ])
        .areas(area);

        let [body_left_sector, body_right_sector] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(body);

        let [keybinds] = Layout::horizontal([Constraint::Percentage(100)]).areas(footer);

        let [project, runs, workspaces] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0), // Takes whatever space is left
            Constraint::Percentage(30),
        ])
        .areas(body_left_sector);

        let [tabs, command_log] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(body_right_sector);

        {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Diff")
                .render(tabs, buf);
        }
        {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Command log")
                .render(command_log, buf);
        }
        {
            let text = Line::from(format!(
                "Apply: a | Plan: p | Destroy: d | State: s | Workspace: w | Keybinds: ? | Quit: q | Version: {}",
                env!("CARGO_PKG_VERSION")
            ));
            Paragraph::new(text).left_aligned().render(keybinds, buf);
        }
    }
}

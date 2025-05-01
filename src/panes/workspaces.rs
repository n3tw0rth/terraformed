use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::components::pane;

use super::PaneWidget;

#[derive(Default)]
pub struct WorkspacesPane;

impl WorkspacesPane {
    pub fn new() -> Self {
        Self
    }
}

impl PaneWidget for WorkspacesPane {
    fn render(&mut self, area: Rect, buf: &mut Buffer, active: bool) {
        pane::new(active).title("[3] Workspaces").render(area, buf);
    }
}

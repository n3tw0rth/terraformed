use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::components::pane;

use super::PaneWidget;

#[derive(Default)]
pub struct RunsPane;

impl RunsPane {
    pub fn new() -> Self {
        Self
    }
}

impl PaneWidget for RunsPane {
    fn render(&mut self, area: Rect, buf: &mut Buffer, active: bool) {
        pane::new(active).title("[2] Runs").render(area, buf);
    }
}

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::components::pane;

use super::PaneWidget;

#[derive(Default)]
pub struct StatusPane;

impl StatusPane {
    pub fn new() -> Self {
        Self
    }
}

impl PaneWidget for StatusPane {
    fn render(&mut self, area: Rect, buf: &mut Buffer, active: bool) {
        pane::new(active).title("[1] Status").render(area, buf);
    }
}

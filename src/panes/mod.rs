use ratatui::{buffer::Buffer, layout::Rect};
use strum::{EnumCount, EnumIter, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumCount, VariantNames)]
#[repr(usize)]
pub enum Pane {
}

pub trait PaneWidget {
    fn render(&mut self, area: Rect, buf: &mut Buffer, active: bool);
}

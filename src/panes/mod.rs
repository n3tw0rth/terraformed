use ratatui::{buffer::Buffer, layout::Rect};
use strum::{EnumCount, EnumIter, VariantNames};

pub mod runs;
pub mod status;
pub mod workspaces;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, EnumCount, VariantNames)]
#[repr(usize)]
pub enum Pane {
    Status = 0,
    Runs = 1,
    Workspaces = 2,
}

pub trait PaneWidget {
    fn render(&mut self, area: Rect, buf: &mut Buffer, active: bool);
}

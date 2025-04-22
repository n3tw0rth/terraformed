use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .vertical_margin(0)
        .split(area);

        {
            let left_area = chunks[0];
            let paragraph = Paragraph::new("LEFT").centered().block(Block::bordered());
            paragraph.render(left_area, buf);
        }
        {
            let right_area = chunks[1];
            let paragraph = Paragraph::new("RIGHT").centered().block(Block::bordered());
            paragraph.render(right_area, buf);
        }
    }
}

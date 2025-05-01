use ratatui::{
    layout,
    style::{Style, Stylize},
    widgets::{Block, BorderType},
};

pub fn new<'a>(is_selected: bool) -> Block<'a> {
    if is_selected {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().green())
            .title_alignment(layout::Alignment::Left)
    } else {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title_alignment(layout::Alignment::Left)
    }
}

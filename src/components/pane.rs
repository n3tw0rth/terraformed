use ratatui::{
    layout,
    style::{Style, Stylize},
    widgets::{Block, BorderType},
};

pub fn new<'a>(is_selected: bool) -> Result<Block<'a>, Box<dyn std::error::Error>> {
    if is_selected {
        Ok(Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().green())
            .title_alignment(layout::Alignment::Left))
    } else {
        Ok(Block::bordered()
            .border_type(BorderType::Rounded)
            .title_alignment(layout::Alignment::Left))
    }
}

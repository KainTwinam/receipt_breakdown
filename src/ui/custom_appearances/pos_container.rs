use iced::{
    Theme,
    border::{self, Border},
    widget::container::Style};


pub fn pos_table_header(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.strong.color.into()),
        text_color: Some(palette.primary.base.color.into()),
        border: border::rounded(4),
        ..Style::default()
    }
}


pub fn pos_table_row(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        text_color: Some(palette.primary.weak.color.into()),
        border: border::rounded(2), 
        ..Style::default()
    }
}
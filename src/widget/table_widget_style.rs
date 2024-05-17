use iced::{color, Background, Color};

use crate::components::theme;

const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 1.5;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Theme(pub theme::Palette);

impl Theme {
    pub fn inner(&self) -> &theme::Palette {
        &self.0
    }
}

/// The appearance of a [`Table`](crate::widget::table_widget::Table).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The border radius of the [`Table`](crate::widget::table_widget::Table).
    pub border_radius: f32,

    /// The border width of the [`Table`](crate::widget::table_widget::Table).
    pub border_width: f32,

    /// The border color of the [`Table`](crate::widget::table_widget::Table).
    pub border_color: Color,

    /// The background of the header of the [`Table`](crate::widget::table_widget::Table).
    pub header_background: Background,

    /// The text color of the header of the [`Table`](crate::widget::table_widget::Table).
    pub header_text_color: Color,

    /// The background of the body of the [`Table`](crate::widget::table_widget::Table).
    pub body_background: Background,

    /// The text color of the body of the [`Table`](crate::widget::table_widget::Table).
    pub body_text_color: Color,

    /// The background of the highlighted cell of the [`Table`](crate::widget::table_widget::Table).
    pub highlighted_cell_background: Background,

    /// The background of the row and column cells of the highlighted cell of the [`Table`](crate::widget::table_widget::Table).
    pub highlighted_row_or_column_background: Background,
}

/// The appearance of a [`Table`](crate::widget::table_widget::Table).
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub trait StyleSheet {
    type Style: Default;
    /// The normal appearance of a [`Table`](crate::widget::table_widget::Table).
    fn active(&self, style: &Self::Style) -> Appearance;
}

#[derive(Default)]
pub enum TableStyle {
    #[default]
    Default,
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl TableStyle {
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Box::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = TableStyle;

    fn active(&self, style: &Self::Style) -> Appearance {

        match style {
            TableStyle::Default => Appearance::default(),
            TableStyle::Custom(style_sheet) => style_sheet.active(self),
        }

    }
}

impl Default for Appearance {
    fn default() -> Self {


        Self {
            border_radius: BORDER_RADIUS,
            border_width: BORDER_WIDTH,
            border_color: color!(0x666666d9),
            header_background: color!(0x666666d9).into(),
            header_text_color: color!(0x111111),
            body_background: Color::TRANSPARENT.into(),
            body_text_color: color!(0x111111),
            highlighted_cell_background: color!(0x66666680).into(),
            highlighted_row_or_column_background: color!(0x6666664d).into(),
        }
    }
}


//// Custom StyleSheet implementation?
// border_radius: BORDER_RADIUS,
// border_width: BORDER_WIDTH,
// border_color: self.inner().border,
// header_background: self.inner().background,
// header_text_color: self.inner().text,
// body_background: self.inner().middleground,
// body_text_color: self.inner().text,
// highlighted_cell_background: self.inner().foreground,
// highlighted_row_or_column_background: self.inner().background,
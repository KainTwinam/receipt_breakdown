use iced::{
    Theme,
    Element,
    border::{self, Border,},
    Background
};
use iced::widget::{text_input, text_input::Style, text_input::Status};

// Our validated input type
#[derive(Debug, Clone)]
pub struct Input {
    pub value: String,
    pub is_valid: bool,
    pub placeholder: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            value: String::new(),
            is_valid: true,
            placeholder: String::new(),
        }
    }
}

impl Input {
    pub fn new(placeholder: &str) -> Self  {
        Self {
            placeholder: placeholder.to_string(),
            ..Input::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    RawInput(String),
    RawSubmit(String),
}

pub fn view<'a>(
    value: &str, 
    placeholder: &str,
    is_valid: bool,
) -> Element<'a, Message> {

    if is_valid {
        text_input(placeholder, value)
        .on_input(Message::RawInput)
        .on_submit(Message::RawSubmit(value.to_string()))
        .width(120)
        .into()
    } else {
        text_input(placeholder, value)
        .on_input(Message::RawInput)
        .on_submit(Message::RawSubmit(value.to_string()))
        .width(120)
        .style(invalid)
        .into()
    }

}

pub fn validate<F>(input: &str, validator: F) -> bool
where
    F: FnOnce(&str) -> bool,
{
    validator(input)
}

pub fn invalid(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        background: Background::Color(palette.danger.base.color),
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.danger.strong.color,
        },
        icon: palette.danger.weak.text,
        placeholder: palette.danger.strong.color,
        value: palette.danger.base.text,
        selection: palette.danger.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: palette.danger.base.text,
                ..active.border
            },
            ..active
        },
        Status::Focused => Style {
            border: Border {
                color: palette.danger.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Background::Color(palette.danger.weak.color),
            value: active.placeholder,
            ..active
        },
    }
}
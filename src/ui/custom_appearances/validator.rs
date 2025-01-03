use iced::Element;
use iced::widget::text_input;

// Our validated input type
#[derive(Debug, Default, Clone)]
pub struct Input {
    pub value: String,
    pub is_valid: bool,
    pub placeholder: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    RawInput(String),
    RawSubmit(String),
}

pub fn view<'a>(value: &str, placeholder: &str) -> Element<'a, Message> {
    text_input(placeholder, value)
        .on_input(Message::RawInput)
        .on_submit(Message::RawSubmit(value.to_string()))
        .into()
}

pub fn validate<F>(input: &str, validator: F) -> bool
where
    F: FnOnce(&str) -> bool,
{
    validator(input)
}

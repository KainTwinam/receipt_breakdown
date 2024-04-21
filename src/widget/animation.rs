//! Displays animated GIFs
//! uses iced_gif

use once_cell::sync::Lazy;
pub static GIF: Lazy<Animation> = Lazy::new(Animation::new);

#[cfg(not(feature = "iced_gif"))]
pub struct Animation;

#[cfg(not(feature = "iced_gif"))]
impl Animation {
    pub fn idle<Message>(&self) -> Option<super::Element<Message>> {
        None
    }
    pub fn ripping<Message>(&self) -> Option<super::Element<Message>> {
        None
    }
    pub fn new() -> Self {
        Self
    }
}

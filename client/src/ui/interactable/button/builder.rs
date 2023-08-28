use std::sync::Arc;

pub struct ButtonBuilder {
    pub text: String,
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        ButtonBuilder {
            text: String::from(""),
        }
    }
}

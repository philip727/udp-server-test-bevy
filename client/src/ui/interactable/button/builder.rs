use super::task::ButtonTask;

pub struct ButtonBuilder {
    pub text: String,
    pub task: ButtonTask,
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        ButtonBuilder {
            text: String::from(""),
            task: ButtonTask::new(|| {}),
        }
    }
}

use bevy::prelude::Component;

pub trait ButtonTaskCallback: Send + Sync {
    fn call(&self);
}

impl<F> ButtonTaskCallback for F
where
    F: Fn() + Send + Sync,
{
    fn call(&self) {
        self();
    }
}

#[derive(Component)]
pub struct ButtonTask {
    function: Box<dyn ButtonTaskCallback>,
}

impl ButtonTask {
    pub fn new<F>(function: F) -> Self
    where
        F: Fn() + 'static + Send + Sync,
    {
        ButtonTask {
            function: Box::new(function),
        }
    }

    pub fn execute(&self) {
        self.function.call();
    }
}

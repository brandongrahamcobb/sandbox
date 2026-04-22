use crate::net::error::NetworkError;

pub struct HandlerResult<T> {
    pub actions: Vec<T>,
}

impl<T> HandlerResult<T> {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(self: &mut Self, action: T) -> Result<(), NetworkError> {
        self.actions.push(action);
        Ok(())
    }
}

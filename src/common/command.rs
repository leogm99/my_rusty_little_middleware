pub struct DefineCommand {
    queue: String,
}

impl DefineCommand {
    pub fn new(queue_name: &str) -> Self {
        DefineCommand {
            queue: String::from(queue_name),
        }
    }

    // Maybe a better solution would be to build another object
    // and implement a trait to make a copy
    pub fn queue_name_as_copy(&self) -> String {
        self.queue.to_string()
    }
}

pub struct PushCommand {
    queue: String,
    message: String,
}

impl PushCommand {
    pub fn new(name: &str, message: &str) -> Self {
        PushCommand {
            queue: String::from(name),
            message: String::from(message),
        }
    }

    pub fn queue_name_as_copy(&self) -> String {
        self.queue.to_string()
    }

    pub fn message_as_copy(&self) -> String {
        self.message.to_string()
    }
}

pub struct PopCommand {
    queue: String,
}

impl PopCommand {
    pub fn new(queue: &str) -> Self {
        PopCommand {
            queue: String::from(queue),
        }
    }

    pub fn queue_name_as_copy(&self) -> String {
        self.queue.to_string()
    }
}

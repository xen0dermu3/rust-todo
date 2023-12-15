#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[allow(dead_code)]
impl Todo {
    pub fn new(title: String, description: Option<String>, completed: Option<bool>) -> Self {
        Todo {
            title,
            description: description.unwrap_or(String::new()),
            completed: completed.unwrap_or(false),
        }
    }

    pub fn mark_as_completed(&mut self, completed: Option<bool>) {
        self.completed = completed.unwrap_or(!self.completed);
    }

    pub fn update(
        &mut self,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) {
        if title.is_some() {
            self.title = title.unwrap()
        }

        if description.is_some() {
            self.description = description.unwrap()
        }

        if completed.is_some() {
            self.completed = completed.unwrap()
        }
    }
}

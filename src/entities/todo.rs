use instant::Instant;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: Instant,
}

impl Todo {
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created = Instant::now();
        Todo { id, title, description, created }
    }

    pub(crate) fn new_empty() -> Todo {
        Self::new("".to_string(), "".to_string())
    }
}
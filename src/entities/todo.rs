use chrono::{DateTime, Utc};
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoJsValue {
    pub todo: Todo
}

impl Todo {
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created = Utc::now();
        Todo { id, title, description, created }
    }

    pub fn new_empty() -> Todo {
        Self::new("".to_string(), "".to_string())
    }
    pub fn js_value(&self) -> JsValue {
        let container = TodoJsValue { todo: self.clone() };
        JsValue::from_serde(&container).unwrap()
    }
}
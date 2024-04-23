use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl User {
    pub fn new(id: &str, name: &str, email: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            name: Some(name.to_string()),
            email: Some(email.to_string()),
        }
    }

    pub fn new_id(id: &str) -> Self {
        Self {
            id: Some(id.to_string()),
            name: None,
            email: None,
        }
    }

    pub fn new_name(name: &str) -> Self {
        Self {
            id: None,
            name: Some(name.to_string()),
            email: None,
        }
    }

    pub fn new_email(email: &str) -> Self {
        Self {
            id: None,
            name: None,
            email: Some(email.to_string()),
        }
    }

    pub fn id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    pub fn name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn email(&mut self, email: &str) {
        self.email = Some(email.to_string());
    }
}

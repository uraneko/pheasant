#[derive(Debug)]
pub struct User {
    user: String,
    password: String,
}

impl User {
    pub fn new(user: String, password: String) -> Self {
        Self { user, password }
    }
}

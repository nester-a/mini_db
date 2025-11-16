pub fn alredy_exists() -> String {
    String::from("user already exists")
}

pub fn not_found() -> String {
    String::from("user not found")
}

pub enum DatabaseError {
    AlreadyExists(String),
    NotFound(String)
}
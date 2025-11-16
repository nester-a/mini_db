use std::collections::HashMap;
use crate::{errors, errors::DatabaseError, models::User};

pub struct UserDatabase {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserDatabase {
    pub fn new() -> Self {
        UserDatabase {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_user(&mut self, username: String, email: String) -> Result<u32, DatabaseError> {
        for (_, user) in &self.users {
            if user.username == username || user.email == email {
                return Err(DatabaseError::AlreadyExists(errors::alredy_exists()));
            }
        }

        let cur_id = self.next_id;

        self.users.insert(
            cur_id,
            User {
                id: cur_id,
                username,
                email,
                active: true,
            },
        );

        self.next_id = cur_id + 1;

        Ok(cur_id)
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn delete_user(&mut self, id: u32) -> Result<(), DatabaseError> {
        match self.users.remove(&id) {
            Some(_) => Ok(()),
            None => Err(DatabaseError::NotFound(errors::not_found())),
        }
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}


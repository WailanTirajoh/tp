use super::entities::{NewUser, UpdateUser};
use serde::{Deserialize, Serialize};

/// Input DTO for creating a user
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
}

impl From<CreateUserInput> for NewUser {
    fn from(input: CreateUserInput) -> Self {
        NewUser {
            name: input.name,
            email: input.email,
            age: input.age,
        }
    }
}

impl From<&CreateUserInput> for NewUser {
    fn from(input: &CreateUserInput) -> Self {
        NewUser {
            name: input.name.clone(),
            email: input.email.clone(),
            age: input.age,
        }
    }
}

/// Input DTO for updating a user
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}

impl From<UpdateUserInput> for UpdateUser {
    fn from(input: UpdateUserInput) -> Self {
        UpdateUser {
            name: input.name,
            email: input.email,
            age: input.age,
        }
    }
}

impl From<&UpdateUserInput> for UpdateUser {
    fn from(input: &UpdateUserInput) -> Self {
        UpdateUser {
            name: input.name.clone(),
            email: input.email.clone(),
            age: input.age,
        }
    }
}

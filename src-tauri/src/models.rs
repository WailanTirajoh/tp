use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}

// Input types for Tauri commands (keep for API compatibility)
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

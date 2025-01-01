use diesel::{prelude::AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct UserEntity {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserCreateEntity {
    pub username: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdateEntity {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

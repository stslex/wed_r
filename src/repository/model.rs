use uuid::Uuid;

#[allow(dead_code)]
pub struct UserResponseDataModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

pub struct UserRequestDataModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct UserRequestDataModel<'a> {
    pub uuid: Uuid,
    pub username: &'a str,
    pub name: &'a str,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserResponseDataModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

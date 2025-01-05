use uuid::Uuid;

pub struct AdminRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

#[derive(Debug)]
pub struct CreateUserRequestModel<'a> {
    pub username: &'a str,
    pub name: &'a str,
}

pub struct CreateUserResponseModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

#[allow(dead_code)]
pub struct AdminResponseModel {
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
}

impl<'a> Into<CreateUserRequestModel<'a>> for &AdminRequestModel<'a> {
    fn into(self) -> CreateUserRequestModel<'a> {
        CreateUserRequestModel {
            username: self.username,
            name: self.name,
        }
    }
}

impl Into<AdminResponseModel> for CreateUserResponseModel {
    fn into(self) -> AdminResponseModel {
        AdminResponseModel {
            uuid: self.uuid,
            username: self.username,
            name: self.name,
        }
    }
}

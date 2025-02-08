pub struct StartRequestModel<'a> {
    pub uuid: &'a str,
    pub username: &'a str,
    pub name: &'a str,
    pub chat_id: &'a i64,
}

pub struct StartResponseModel {
    pub messege: String,
    pub is_admin: bool,
}

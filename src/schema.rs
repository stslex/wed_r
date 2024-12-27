diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Varchar,
        name -> Varchar,
    }
}

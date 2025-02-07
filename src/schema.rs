diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Varchar,
        name -> Varchar,
        is_active -> Bool,
        is_accepted -> Bool
    }
}

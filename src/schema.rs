diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        #[max_length = 128]
        username -> Varchar,
        name -> Text,
        chat_id -> Nullable<Int8>,
        is_accepted -> Bool,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    auth (user_id) {
        user_id -> Int4,
        #[max_length = 25]
        passwd -> Varchar,
    }
}

diesel::table! {
    chats (id) {
        id -> Nullable<Int8>,
        from_ -> Int4,
        to_ -> Int4,
        #[max_length = 255]
        msg -> Varchar,
        sent -> Timestamp,
        received -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Int4>,
        #[max_length = 70]
        username -> Varchar,
        #[max_length = 90]
        email_id -> Varchar,
        dob -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    chats,
    users,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Date,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(articles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(articles, users,);

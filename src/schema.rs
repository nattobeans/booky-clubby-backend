// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Int4,
        message -> Text,
        member_id -> Int4,
        to_member_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    group_chats (id) {
        id -> Int4,
        message -> Text,
        group_id -> Int4,
        to_member_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    group_members (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        current_book_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        review -> Text,
        member_id -> Int4,
        book_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(group_chats -> groups (group_id));
diesel::joinable!(group_chats -> members (to_member_id));
diesel::joinable!(group_members -> groups (group_id));
diesel::joinable!(group_members -> members (user_id));
diesel::joinable!(groups -> books (current_book_id));
diesel::joinable!(reviews -> books (book_id));
diesel::joinable!(reviews -> members (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    chats,
    group_chats,
    group_members,
    groups,
    members,
    reviews,
);

table! {
    refresh_tokens (id) {
        id -> Int8,
        token -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        is_admin -> Bool,
        password -> Nullable<Varchar>,
        email_confirmed -> Bool,
        email_confirmation_token -> Nullable<Varchar>,
        email_confirmation_token_sent_at -> Nullable<Timestamp>,
        email_confirmed_at -> Nullable<Timestamp>,
        phone_confirmed -> Bool,
        phone_confirmation_token -> Nullable<Varchar>,
        phone_confirmation_token_sent_at -> Nullable<Timestamp>,
        phone_confirmed_at -> Nullable<Timestamp>,
        recovery_token -> Nullable<Varchar>,
        recovery_token_sent_at -> Nullable<Timestamp>,
        email_change_token -> Nullable<Varchar>,
        new_email -> Nullable<Varchar>,
        email_change_token_sent_at -> Nullable<Timestamp>,
        last_signin_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(refresh_tokens, users);

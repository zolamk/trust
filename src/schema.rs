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
        email -> Varchar,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        is_admin -> Bool,
        password -> Nullable<Varchar>,
        confirmed -> Bool,
        invitation_sent_at -> Nullable<Timestamp>,
        confirmation_token -> Nullable<Varchar>,
        confirmation_token_sent_at -> Nullable<Timestamp>,
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

allow_tables_to_appear_in_same_query!(refresh_tokens, users,);

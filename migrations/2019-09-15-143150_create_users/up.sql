create extension citext;

create table users (
    id bigserial primary key,
    email citext not null constraint uq_email unique,
    aud varchar(250) not null,
    name varchar,
    avatar varchar,
    is_admin boolean not null default false,
    password varchar(82) null,
    confirmed boolean not null default false,
    invitation_sent_at timestamp,
    confirmation_token varchar(250),
    confirmation_token_sent_at timestamp,
    recovery_token varchar(250),
    recovery_token_sent_at timestamp,
    email_change_token varchar(250),
    new_email citext,
    email_change_token_sent_at timestamp,
    last_signin_at timestamp,
    app_metadata json,
    user_metadata json,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null
);

create or replace function update_user_modified_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create trigger update_user_modified_at_trigger before insert or update on users for each row execute procedure update_user_modified_at();
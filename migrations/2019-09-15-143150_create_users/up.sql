create table users (
    id bigserial primary key,
    instance_id varchar(250) not null,
    name varchar(250),
    email varchar(250) not null constraint uq_email unique,
    avatar text,
    aud varchar(250) not null,
    role varchar(250),
    password varchar(82) null,
    confirmed boolean not null default false,
    invited_at timestamp,
    confirmation_token varchar(250),
    confirmation_sent_at timestamp,
    recovery_token varchar(250),
    recovery_sent_at timestamp,
    email_change_token varchar(250),
    email_change varchar(250),
    email_change_sent_at timestamp,
    last_signin_at timestamp,
    app_metadata json,
    user_metadata json,
    is_super_admin boolean not null default false,
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
create table refresh_tokens (
    id bigserial primary key,
    token varchar(250) not null,
    user_id bigint not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null,
    constraint fk_refresh_token_user_id foreign key(user_id) references users(id)
);

create or replace function update_refresh_token_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create trigger update_refresh_token_updated_at_trigger before insert or update on refresh_tokens for each row execute procedure update_refresh_token_updated_at();
create table trust.refresh_tokens (
    id bigserial primary key,
    token varchar not null,
    user_id varchar not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null,
    constraint fk_refresh_token_user_id foreign key(user_id) references trust.users(id) on delete cascade on update cascade
);

create or replace function trust.update_refresh_token_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create trigger update_refresh_token_updated_at_trigger before insert or update on trust.refresh_tokens for each row execute procedure trust.update_refresh_token_updated_at();
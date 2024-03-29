CREATE SCHEMA IF NOT EXISTS trust;

CREATE EXTENSION IF NOT EXISTS citext;

CREATE DOMAIN trust.email AS citext CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE DOMAIN trust.phone_number AS citext CHECK (value ~ '^\+\d{5,15}$');

CREATE OR REPLACE FUNCTION trust.feistel_crypt(value integer)
  RETURNS integer
  LANGUAGE plpgsql
  IMMUTABLE STRICT
AS $function$
DECLARE
    key numeric;
    l1 int;
    l2 int;
    r1 int;
    r2 int;
    i int:=0;
BEGIN
    l1:= (VALUE >> 16) & 65535;
    r1:= VALUE & 65535;
    WHILE i < 3 LOOP
        -- key can be any function that returns numeric between 0 and 1
        key := (((1366 * r1 + 150889) % 714025) / 714025.0);
        l2 := r1;
        r2 := l1 # (key * 32767)::int;
        l1 := l2;
        r1 := r2;
        i := i + 1;
    END LOOP;
    RETURN ((r1 << 16) + l1);
END;
$function$;

CREATE OR REPLACE FUNCTION trust.int_to_string(n int)
  RETURNS text
  LANGUAGE plpgsql
  IMMUTABLE STRICT
AS $function$
DECLARE
    alphabet text:='QRBCF123JKLO45GHIJKLOSTU08MNVW67XAPYZ9';
    base int:=length(alphabet);
    output text:='';
BEGIN
    LOOP
        output := output || substr(alphabet, 1+(n%base)::int, 1);
        n := n / base;
        EXIT WHEN n=0;
    END LOOP;
    RETURN output;
END $function$;

CREATE SEQUENCE trust.users_id_seq AS BIGINT INCREMENT 1 START 1;

CREATE OR REPLACE FUNCTION trust.id(n int)
    RETURNS text
    LANGUAGE plpgsql
    IMMUTABLE STRICT
AS $function$
BEGIN
    RETURN trust.int_to_string(trust.feistel_crypt(n));
END $function$;

create table trust.users (
    id varchar primary key default trust.id(nextval('trust.users_id_seq')::int),
    email trust.email constraint uq_email unique,
    phone trust.phone_number constraint uq_phone unique,
    name varchar,
    avatar varchar,
    is_admin boolean not null default false,
    password varchar(82) null,
    email_confirmed boolean not null default false,
    email_confirmation_token varchar(250),
    email_confirmation_token_sent_at timestamp,
    email_confirmed_at timestamp,
    phone_confirmed boolean not null default false,
    phone_confirmation_token varchar(10),
    phone_confirmation_token_sent_at timestamp,
    phone_confirmed_at timestamp,
    recovery_token varchar(250),
    recovery_token_sent_at timestamp,
    email_change_token varchar(250),
    new_email trust.email,
    email_change_token_sent_at timestamp,
    new_phone trust.phone_number,
    phone_change_token varchar(250),
    phone_change_token_sent_at timestamp,
    last_signin_at timestamp,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null,
    constraint chk_email_or_phone_not_null check (email is not null or phone is not null),
    constraint chk_email_confirm check (email_confirmed = false or email_confirmed_at is not null),
    constraint chk_phone_confirm check (phone_confirmed = false or phone_confirmed_at is not null)
);

create or replace function trust.update_user_modified_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create trigger update_user_modified_at_trigger before insert or update on trust.users for each row execute procedure trust.update_user_modified_at();
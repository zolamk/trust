ALTER TABLE trust.users ADD COLUMN incorrect_login_attempts smallint not null default 0;
ALTER TABLE trust.users ADD COLUMN last_incorrect_login_attempt_at timestamptz;
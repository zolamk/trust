ALTER TABLE trust.users RENAME COLUMN recovery_token TO email_recovery_token;
ALTER TABLE trust.users RENAME COLUMN recovery_token_sent_at TO email_recovery_token_sent_at;
ALTER TABLE trust.users ADD COLUMN phone_recovery_token varchar;
ALTER TABLE trust.users ADD COLUMN phone_recovery_token_sent_at timestamptz;
-- Your SQL goes here
ALTER TABLE trust.users ALTER email_confirmation_token_sent_at TYPE timestamptz USING email_confirmation_token_sent_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER phone_confirmation_token_sent_at TYPE timestamptz USING phone_confirmation_token_sent_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER phone_confirmed_at TYPE timestamptz USING phone_confirmed_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER recovery_token_sent_at TYPE timestamptz USING recovery_token_sent_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER email_change_token_sent_at TYPE timestamptz USING email_change_token_sent_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER phone_change_token_sent_at TYPE timestamptz USING phone_change_token_sent_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER last_signin_at TYPE timestamptz USING last_signin_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER last_signin_at TYPE timestamptz USING last_signin_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE trust.users ALTER email_confirmed_at TYPE timestamptz USING email_confirmed_at AT TIME ZONE 'UTC';
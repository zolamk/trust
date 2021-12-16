-- Your SQL goes here
ALTER TABLE trust.users ADD COLUMN email_invitation_token VARCHAR;
ALTER TABLE trust.users ADD COLUMN phone_invitation_token VARCHAR;
ALTER TABLE trust.users ADD COLUMN invitation_token_sent_at TIMESTAMPTZ;
ALTER TABLE trust.users ADD COLUMN invitation_accepted_at TIMESTAMPTZ;
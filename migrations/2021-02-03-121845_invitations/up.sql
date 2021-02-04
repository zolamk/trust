-- Your SQL goes here
ALTER TABLE users ADD COLUMN email_invitation_token VARCHAR;
ALTER TABLE users ADD COLUMN phone_invitation_token VARCHAR;
ALTER TABLE users ADD COLUMN invitation_token_sent_at TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN invitation_accepted_on TIMESTAMPTZ;
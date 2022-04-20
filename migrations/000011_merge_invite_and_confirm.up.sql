ALTER TABLE trust.users
    DROP COLUMN email_invitation_token,
    DROP COLUMN phone_invitation_token,
    DROP COLUMN invitation_token_sent_at,
    DROP COLUMN invitation_accepted_at;
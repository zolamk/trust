ALTER TABLE trust.users ADD CONSTRAINT uq_email_confirmation_token UNIQUE(email_confirmation_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_phone_confirmation_token UNIQUE(phone_confirmation_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_recovery_token UNIQUE(recovery_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_email_change_token UNIQUE(email_change_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_phone_change_token UNIQUE(phone_change_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_email_invitation_token UNIQUE(email_invitation_token);
ALTER TABLE trust.users ADD CONSTRAINT uq_phone_invitation_token UNIQUE(phone_invitation_token);
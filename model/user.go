package model

import (
	"time"

	"gorm.io/gorm"
)

type User struct {
	ID                           string     `json:"id,omitempty" gorm:"autoIncrement"`
	Email                        *string    `json:"email,omitempty"`
	Phone                        *string    `json:"phone,omitempty"`
	Name                         *string    `json:"name,omitempty"`
	Avatar                       *string    `json:"avatar,omitempty"`
	EmailConfirmed               bool       `json:"email_confirmed,omitempty"`
	EmailConfirmationTokenSentAt *time.Time `json:"email_confirmation_token_sent_at,omitempty"`
	EmailConfirmedAt             *time.Time `json:"email_confirmed_at,omitempty"`
	PhoneConfirmed               bool       `json:"phone_confirmed,omitempty"`
	PhoneConfirmationTokenSentAt *time.Time `json:"phone_confirmation_token_sent_at,omitempty"`
	PhoneConfirmedAt             *time.Time `json:"phone_confirmed_at,omitempty"`
	EmailRecoveryTokenSentAt     *time.Time `json:"email_recovery_token_sent_at,omitempty"`
	PhoneRecoveryTokenSentAt     *time.Time `json:"phone_recovery_token_sent_at,omitempty"`
	EmailChangeTokenSentAt       *time.Time `json:"email_change_token_sent_at,omitempty"`
	PhoneChangeTokenSentAt       *time.Time `json:"phone_change_token_sent_at,omitempty"`
	LastSigninAt                 *time.Time `json:"last_signin_at,omitempty"`
	CreatedAt                    time.Time  `json:"created_at,omitempty"`
	UpdatedAt                    time.Time  `json:"updated_at,omitempty"`
	InvitationTokenSentAt        *time.Time `json:"invitation_token_sent_at,omitempty"`
	InvitationAcceptedAt         *time.Time `json:"invitation_accepted_at,omitempty"`
	IsAdmin                      bool       `json:"-"`
	Password                     *string    `json:"-"`
	EmailConfirmationToken       *string    `json:"-"`
	PhoneConfirmationToken       *string    `json:"-"`
	EmailRecoveryToken           *string    `json:"-"`
	PhoneRecoveryToken           *string    `json:"-"`
	EmailChangeToken             *string    `json:"-"`
	NewEmail                     *string    `json:"new_email,omitempty"`
	NewPhone                     *string    `json:"new_phone,omitempty"`
	EmailInvitationToken         *string    `json:"-"`
	PhoneInvitationToken         *string    `json:"-"`
}

func (u *User) Create(db *gorm.DB) error {
	return db.Create(u).Error
}

func (u *User) Save(db *gorm.DB) error {
	return db.Save(u).Error
}

func (u *User) ConfirmEmail(db *gorm.DB) error {

	now := time.Now()

	u.EmailConfirmed = true

	u.EmailConfirmedAt = &now

	u.EmailConfirmationToken = nil

	return db.Save(u).Error

}

func (u *User) ConfirmPhone(db *gorm.DB) error {

	now := time.Now()

	u.PhoneConfirmed = true

	u.PhoneConfirmedAt = &now

	u.PhoneConfirmationToken = nil

	return db.Save(u).Error

}

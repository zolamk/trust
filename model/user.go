package model

import (
	"time"

	"golang.org/x/crypto/bcrypt"
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
	RecoveryTokenSentAt          *time.Time `json:"recovery_token_sent_at,omitempty"`
	EmailChangeTokenSentAt       *time.Time `json:"email_change_token_sent_at,omitempty"`
	PhoneChangeTokenSentAt       *time.Time `json:"phone_change_token_sent_at,omitempty"`
	LastSigninAt                 *time.Time `json:"last_signin_at,omitempty"`
	CreatedAt                    time.Time  `json:"created_at,omitempty"`
	UpdatedAt                    time.Time  `json:"updated_at,omitempty"`
	InvitationTokenSentAt        *time.Time `json:"invitation_token_sent_at,omitempty"`
	InvitationAcceptedAt         *time.Time `json:"invitation_accepted_at,omitempty"`
	NewEmail                     *string    `json:"new_email,omitempty"`
	NewPhone                     *string    `json:"new_phone,omitempty"`
	PhoneChangedAt               *time.Time `json:"phone_changed_at,omitempty"`
	EmailChangedAt               *time.Time `json:"email_changed_at,omitempty"`
	PasswordChangedAt            *time.Time `json:"password_changed_at,omitempty"`
	IncorrectLoginAttempts       uint8      `json:"-"`
	LastIncorrectLoginAttemptAt  *time.Time `json:"last_incorrect_login_attempt_at"`
	IsAdmin                      bool       `json:"-"`
	Password                     *string    `json:"-"`
	EmailConfirmationToken       *string    `json:"-"`
	PhoneConfirmationToken       *string    `json:"-"`
	RecoveryToken                *string    `json:"-"`
	EmailChangeToken             *string    `json:"-"`
	PhoneChangeToken             *string    `json:"-"`
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

	return u.Save(db)

}

func (u *User) ConfirmPhone(db *gorm.DB) error {

	now := time.Now()

	u.PhoneConfirmed = true

	u.PhoneConfirmedAt = &now

	u.PhoneConfirmationToken = nil

	return u.Save(db)

}

func (u *User) ConfirmPhoneChange(db *gorm.DB) error {

	now := time.Now()

	u.Phone = u.NewPhone

	u.NewPhone = nil

	u.PhoneChangedAt = &now

	u.PhoneChangeToken = nil

	u.PhoneChangeTokenSentAt = nil

	return u.Save(db)

}

func (u *User) ConfirmEmailChange(db *gorm.DB) error {

	now := time.Now()

	u.Email = u.NewEmail

	u.NewEmail = nil

	u.EmailChangedAt = &now

	u.EmailChangeToken = nil

	u.EmailChangeTokenSentAt = nil

	return u.Save(db)

}

func (u *User) AcceptPhoneInvite(db *gorm.DB) error {

	now := time.Now()

	u.PhoneInvitationToken = nil

	u.InvitationAcceptedAt = &now

	u.PhoneConfirmedAt = &now

	u.PhoneConfirmed = true

	return u.Save(db)

}

func (u *User) AcceptEmailInvite(db *gorm.DB) error {

	now := time.Now()

	u.EmailInvitationToken = nil

	u.InvitationAcceptedAt = &now

	u.EmailConfirmedAt = &now

	u.EmailConfirmed = true

	return u.Save(db)

}

func (u *User) SignedIn(db *gorm.DB, log *Log) error {

	now := time.Now()

	u.LastSigninAt = &now

	u.IncorrectLoginAttempts = 0

	if tx := db.Create(log); tx.Error != nil {
		return tx.Error
	}

	return u.Save(db)

}

func (u *User) ResetAttempt(db *gorm.DB) error {

	u.IncorrectLoginAttempts = 0

	return u.Save(db)

}

func (u *User) IncorrectAttempt(db *gorm.DB, log *Log) error {

	now := time.Now()

	u.IncorrectLoginAttempts++

	u.LastIncorrectLoginAttemptAt = &now

	if tx := db.Create(log); tx.Error != nil {
		return tx.Error
	}

	return u.Save(db)

}

func (u *User) SetPassword(password string, cost int) error {

	pwd, err := bcrypt.GenerateFromPassword([]byte(password), cost)

	if err != nil {
		return err
	}

	hash := string(pwd)

	u.Password = &hash

	return nil

}

func (u *User) VerifyPassword(password string) error {

	if u.Password == nil {
		return bcrypt.ErrMismatchedHashAndPassword
	}

	return bcrypt.CompareHashAndPassword([]byte(*u.Password), []byte(password))

}

package model

import (
	"time"

	"github.com/thanhpk/randstr"
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

func (u *User) Save(tx *gorm.DB) error {
	return tx.Save(u).Error
}

func (u *User) SaveWithLog(tx *gorm.DB, log *Log) error {

	if err := tx.Create(log).Error; err != nil {
		return err
	}

	return tx.Save(u).Error

}

func (u *User) InviteByEmail(tx *gorm.DB, name string, email string) error {

	now := time.Now()

	invitation_token := randstr.String(100)

	u.Name = &name

	u.Email = &email

	u.EmailInvitationToken = &invitation_token

	u.InvitationTokenSentAt = &now

	return tx.Save(u).Error

}

func (u *User) InviteByPhone(tx *gorm.DB, name string, phone string) error {

	now := time.Now()

	invitation_token := randstr.String(6)

	u.Name = &name

	u.Phone = &phone

	u.PhoneInvitationToken = &invitation_token

	u.InvitationTokenSentAt = &now

	return tx.Save(u).Error

}

func (u *User) ResetByEmail(tx *gorm.DB, log *Log) error {

	token := randstr.String(100)

	now := time.Now()

	u.RecoveryToken = &token

	u.RecoveryTokenSentAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ResetByPhone(tx *gorm.DB, log *Log) error {

	token := randstr.String(6)

	now := time.Now()

	u.RecoveryToken = &token

	u.RecoveryTokenSentAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ChangePassword(tx *gorm.DB, log *Log, password string, cost int) error {

	now := time.Now()

	if err := u.SetPassword(password, cost); err != nil {
		return err
	}

	u.PasswordChangedAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ChangeEmail(tx *gorm.DB, log *Log, email string) error {

	now := time.Now()

	token := randstr.String(100)

	u.EmailChangeToken = &token

	u.NewEmail = &email

	u.EmailChangeTokenSentAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ChangePhone(tx *gorm.DB, log *Log, phone string) error {

	now := time.Now()

	token := randstr.String(6)

	u.PhoneChangeToken = &token

	u.NewPhone = &phone

	u.PhoneChangeTokenSentAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ConfirmReset(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.RecoveryToken = nil

	u.RecoveryTokenSentAt = nil

	u.PasswordChangedAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ConfirmEmail(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.EmailConfirmed = true

	u.EmailConfirmedAt = &now

	u.EmailConfirmationToken = nil

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ConfirmPhone(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.PhoneConfirmed = true

	u.PhoneConfirmedAt = &now

	u.PhoneConfirmationToken = nil

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ConfirmPhoneChange(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.Phone = u.NewPhone

	u.NewPhone = nil

	u.PhoneChangedAt = &now

	u.PhoneChangeToken = nil

	u.PhoneChangeTokenSentAt = nil

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ConfirmEmailChange(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.Email = u.NewEmail

	u.NewEmail = nil

	u.EmailChangedAt = &now

	u.EmailChangeToken = nil

	u.EmailChangeTokenSentAt = nil

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) AcceptPhoneInvite(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.PhoneInvitationToken = nil

	u.InvitationAcceptedAt = &now

	u.PhoneConfirmedAt = &now

	u.PhoneConfirmed = true

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) AcceptEmailInvite(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.EmailInvitationToken = nil

	u.InvitationAcceptedAt = &now

	u.EmailConfirmedAt = &now

	u.EmailConfirmed = true

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) SignedIn(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.LastSigninAt = &now

	u.IncorrectLoginAttempts = 0

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

}

func (u *User) ResetAttempt(tx *gorm.DB) error {

	u.IncorrectLoginAttempts = 0

	return tx.Save(u).Error

}

func (u *User) IncorrectAttempt(tx *gorm.DB, log *Log) error {

	now := time.Now()

	u.IncorrectLoginAttempts++

	u.LastIncorrectLoginAttemptAt = &now

	if tx := tx.Create(log); tx.Error != nil {
		return tx.Error
	}

	return tx.Save(u).Error

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

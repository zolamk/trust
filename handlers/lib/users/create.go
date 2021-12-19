package users

import (
	"errors"
	"time"

	"github.com/jackc/pgconn"
	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func CreateUser(db *gorm.DB, config *config.Config, token *jwt.JWT, form model.CreateUserForm) (*model.User, error) {

	is_admin, err := token.IsAdmin(db)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if form.Email == nil && form.Phone == nil {
		return nil, handlers.ErrEmailOrPhoneRequired
	}

	if form.Email != nil && !config.EmailRule.MatchString(*form.Email) {
		return nil, handlers.ErrInvalidEmail
	}

	if form.Phone != nil && !config.PhoneRule.MatchString(*form.Phone) {
		return nil, handlers.ErrInvalidPhone
	}

	user := &model.User{
		Name:   form.Name,
		Avatar: form.Avatar,
		Email:  form.Email,
		Phone:  form.Phone,
	}

	if form.Password != nil {

		if !config.PasswordRule.MatchString(*form.Password) {
			return nil, handlers.ErrInvalidPassword
		}

		password, err := bcrypt.GenerateFromPassword([]byte(*form.Password), int(config.PasswordHashCost))

		if err != nil {
			logrus.Error(err)
			return nil, handlers.ErrInternal
		}

		hash := string(password)

		user.Password = &hash

	}

	err = db.Transaction(func(tx *gorm.DB) error {

		if err := user.Create(tx); err != nil {

			var pgerr *pgconn.PgError

			if errors.As(err, &pgerr) {

				if pgerr.ConstraintName == "uq_email" {
					return handlers.ErrEmailRegistered
				}

				if pgerr.ConstraintName == "uq_phone" {
					return handlers.ErrPhoneRegistered
				}

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if config.AutoConfirm || (form.Confirm != nil && *form.Confirm) {

			if user.Email != nil {

				if err := user.ConfirmEmail(tx); err != nil {
					logrus.Error(err)
					return handlers.ErrInternal
				}

			}

			if user.Phone != nil {

				if err := user.ConfirmPhone(tx); err != nil {
					logrus.Error(err)
					return handlers.ErrInternal
				}

			}

			return nil

		}

		now := time.Now()

		if user.Email != nil {

			token := randstr.String(100)

			user.EmailConfirmationToken = &token

			user.EmailConfirmationTokenSentAt = &now

			if err := user.Save(tx); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			context := &map[string]string{
				"site_url":                 config.SiteURL,
				"email_confirmation_token": *user.EmailConfirmationToken,
				"instance_url":             config.InstanceURL,
			}

			if err := email.SendEmail(config.ConfirmationTemplate, context, user.Email, config); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

		}

		if user.Phone != nil {

			token := randstr.String(6)

			user.PhoneConfirmationToken = &token

			user.PhoneConfirmationTokenSentAt = &now

			if err := user.Save(tx); err != nil {
				return err
			}

			context := &map[string]string{
				"site_url":                 config.SiteURL,
				"phone_confirmation_token": *user.PhoneConfirmationToken,
				"instance_url":             config.InstanceURL,
			}

			if err := sms.SendSMS(config.ConfirmationTemplate, user.Phone, context, config.SMS); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			return nil

		}

		return nil

	})

	if err != nil {
		return nil, err
	}

	return user, nil
}

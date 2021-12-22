package anonymous

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Signup(db *gorm.DB, config *config.Config, form model.SignupForm) (*model.User, error) {

	now := time.Now()

	user := &model.User{
		Name:   form.Name,
		Email:  form.Email,
		Phone:  form.Phone,
		Avatar: form.Avatar,
	}

	if !config.PasswordRule.MatchString(form.Password) {
		return nil, handlers.ErrInvalidPassword
	}

	if form.Email != nil {

		if !config.EmailRule.MatchString(*form.Email) {
			return nil, handlers.ErrInvalidEmail
		}

		if config.DisableEmail {
			return nil, handlers.ErrEmailDisabled
		}

		tx := db.First(user, "email = ?", *form.Email)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, handlers.ErrInternal
			}
		} else {
			return nil, handlers.ErrEmailRegistered
		}

	}

	if form.Phone != nil {

		if !config.PhoneRule.MatchString(*form.Phone) {
			return nil, handlers.ErrInvalidPhone
		}

		if config.DisablePhone {
			return nil, handlers.ErrPhoneDisabled
		}

		tx := db.First(user, "phone = ?", *form.Phone)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, handlers.ErrInternal
			}
		} else {
			return nil, handlers.ErrPhoneRegistered
		}

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		user.SetPassword(form.Password, int(config.PasswordHashCost))

		if err := user.Create(tx); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

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

			if err := mail.SendEmail(config.ConfirmationTemplate, context, user.Email, config); err != nil {
				return err
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
				return err
			}

			return nil

		}

		return nil

	})

	return user, err

}

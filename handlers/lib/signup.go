package lib

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func Signup(db *gorm.DB, config *config.Config, form model.SignupForm) (*model.User, error) {

	now := time.Now()

	user := &model.User{
		Name:     form.Name,
		Email:    form.Email,
		Phone:    form.Phone,
		Avatar:   form.Avatar,
		Password: &form.Password,
	}

	if !config.PasswordRule.MatchString(form.Password) {
		return nil, errors.InvalidPassword
	}

	if form.Email != nil {

		if !config.EmailRule.MatchString(*form.Email) {
			return nil, errors.InvalidEmail
		}

		if config.DisableEmail {
			return nil, errors.EmailDisabled
		}

		tx := db.First(user, "email = ?", *form.Email)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, errors.Internal
			}
		} else {
			return nil, errors.EmailRegistered
		}

	}

	if form.Phone != nil {

		if !config.PhoneRule.MatchString(*form.Phone) {
			return nil, errors.InvalidPhone
		}

		if config.DisablePhone {
			return nil, errors.PhoneDisabled
		}

		tx := db.First(user, "phone = ?", *form.Phone)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, errors.Internal
			}
		} else {
			return nil, errors.PhoneRegistered
		}

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		password, err := bcrypt.GenerateFromPassword([]byte(form.Password), int(config.PasswordHashCost))

		if err != nil {
			logrus.Error(err)
			return errors.Internal
		}

		hash := string(password)

		user.Password = &hash

		if err := user.Create(tx); err != nil {
			logrus.Error(err)
			return errors.Internal
		}

		if user.Email != nil {

			if config.AutoConfirm {

				if err := user.ConfirmEmail(tx); err != nil {
					logrus.Error(err)
					return errors.Internal
				}

			} else {

				token := randstr.String(100)

				user.EmailConfirmationToken = &token

				user.EmailConfirmationTokenSentAt = &now

				if err := user.Save(tx); err != nil {
					logrus.Error(err)
					return errors.Internal
				}

				context := &map[string]string{
					"site_url":                 config.SiteURL,
					"email_confirmation_token": *user.EmailConfirmationToken,
					"instance_url":             config.InstanceURL,
				}

				if err := email.SendEmail(config.ConfirmationTemplate, context, user.Email, config); err != nil {
					return err
				}

			}

		}

		if user.Phone != nil {

			if config.AutoConfirm {

				if err := user.ConfirmPhone(tx); err != nil {
					return err
				}

				return nil

			} else {

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

		}

		return nil

	})

	return user, err

}

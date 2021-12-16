package lib

import (
	"errors"
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func Signup(db *gorm.DB, config *config.Config, form model.SignupForm) (*model.User, error) {

	now := time.Now()

	internal_error := errors.New("internal server error")

	user := &model.User{
		Name:     form.Name,
		Email:    form.Email,
		Phone:    form.Phone,
		Avatar:   form.Avatar,
		Password: &form.Password,
	}

	if form.Email != nil {

		if !config.EmailRule.MatchString(*form.Email) {
			return nil, errors.New("invalid email address")
		}

		if config.DisableEmail {
			return nil, errors.New("email signup disabled")
		}

		tx := db.First(user, "email = ?", *form.Email)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, internal_error
			}
		} else {
			return nil, errors.New("email already registered")
		}

	}

	if form.Phone != nil {

		if !config.PhoneRule.MatchString(*form.Phone) {
			return nil, errors.New("invalid phone number")
		}

		if config.DisablePhone {
			return nil, errors.New("phone signup disabled")
		}

		tx := db.First(user, "phone = ?", *form.Phone)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, internal_error
			}
		} else {
			return nil, errors.New("phone already registered")
		}

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		password, err := bcrypt.GenerateFromPassword([]byte(form.Password), int(config.PasswordHashCost))

		if err != nil {
			logrus.Error(err)
			return internal_error
		}

		hash := string(password)

		user.Password = &hash

		if err := user.Create(tx); err != nil {
			logrus.Error(err)
			return internal_error
		}

		if user.Email != nil {

			if config.AutoConfirm {

				if err := user.ConfirmEmail(tx); err != nil {
					logrus.Error(err)
					return internal_error
				}

			} else {

				token := randstr.String(100)

				user.EmailConfirmationToken = &token

				user.EmailConfirmationTokenSentAt = &now

				if err := user.Save(tx); err != nil {
					logrus.Error(err)
					return internal_error
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

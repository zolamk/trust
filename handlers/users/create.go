package users

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func CreateUser(db *gorm.DB, config *config.Config, token *jwt.JWT, form model.CreateUserForm, log_data *middleware.LogData) (*model.User, error) {

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

	user := &model.User{
		Name:   form.Name,
		Avatar: form.Avatar,
		Email:  form.Email,
		Phone:  form.Phone,
	}

	if form.Email != nil {

		if !config.EmailRule.MatchString(*form.Email) {
			return nil, handlers.ErrInvalidEmail
		}

		tx := db.First(user, "email = ?", *form.Email)

		if tx.Error != nil {

			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, handlers.ErrInternal
			}

		} else {

			err := handlers.ErrEmailRegistered

			err.Extensions["email"] = user.Email

			err.Extensions["phone"] = user.Phone

			err.Extensions["id"] = user.ID

			return nil, err

		}

	}

	if form.Phone != nil {

		if !config.PhoneRule.MatchString(*form.Phone) {
			return nil, handlers.ErrInvalidPhone
		}

		tx := db.First(user, "phone = ?", *form.Phone)

		if tx.Error != nil {
			if tx.Error != gorm.ErrRecordNotFound {
				logrus.Error(tx.Error)
				return nil, handlers.ErrInternal
			}
		} else {

			err := handlers.ErrPhoneRegistered

			err.Extensions["email"] = user.Email

			err.Extensions["phone"] = user.Phone

			err.Extensions["id"] = user.ID

			return nil, err

		}

	}

	if form.Password != nil {

		if !config.PasswordRule.MatchString(*form.Password) {
			return nil, handlers.ErrInvalidPassword
		}

		user.SetPassword(*form.Password, int(config.PasswordHashCost))

	}

	err = db.Transaction(func(tx *gorm.DB) error {

		if err := user.Create(tx); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		if form.Confirm != nil && *form.Confirm {

			if user.Email != nil {

				log := model.NewLog(user.ID, "email confirmed by admin", log_data.IP, &token.Subject, log_data.Location, log_data.UserAgent)

				if err := user.ConfirmEmail(tx, log); err != nil {
					logrus.Error(err)
					return handlers.ErrInternal
				}

			}

			if user.Phone != nil {

				log := model.NewLog(user.ID, "phone confirmed by admin", log_data.IP, &token.Subject, log_data.Location, log_data.UserAgent)

				if err := user.ConfirmPhone(tx, log); err != nil {

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

			if err := mail.SendEmail(config.ConfirmationTemplate, context, user.Email, config); err != nil {
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

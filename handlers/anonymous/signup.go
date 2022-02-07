package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Signup(db *gorm.DB, config *config.Config, form model.SignupForm) (*model.User, error) {

	var err error

	err = form.Data.Validate(config.CustomDataSchema)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrObjectDoesntMatchSchema
	}

	if !config.PasswordRule.MatchString(form.Password) {
		return nil, handlers.ErrInvalidPassword
	}

	user := &model.User{
		Name:   form.Name,
		Email:  form.Email,
		Phone:  form.Phone,
		Avatar: form.Avatar,
		Data:   &form.Data,
	}

	if form.Email != nil {

		if !config.EmailRule.MatchString(*form.Email) {
			return nil, handlers.ErrInvalidEmail
		}

		if config.DisableEmail {
			return nil, handlers.ErrEmailDisabled
		}

		err = db.First(user, "email = ?", *form.Email).Error

		if err != nil {
			if err != gorm.ErrRecordNotFound {

				logrus.Error(err)

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

		err = db.First(user, "phone = ?", *form.Phone).Error

		if err != nil {

			if err != gorm.ErrRecordNotFound {

				logrus.Error(err)

				return nil, handlers.ErrInternal

			}

		} else {

			return nil, handlers.ErrPhoneRegistered

		}

	}

	err = db.Transaction(func(tx *gorm.DB) error {

		user.SetPassword(form.Password, int(config.PasswordHashCost))

		if err = user.Create(tx); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if user.Email != nil {

			if err = user.InitEmailConfirmation(tx); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			context := &map[string]string{
				"site_url":                 config.SiteURL,
				"email_confirmation_token": *user.EmailConfirmationToken,
				"instance_url":             config.InstanceURL,
			}

			if err = mail.SendEmail(config.ConfirmationTemplate, context, user.Email, config); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		}

		if user.Phone != nil {

			if err = user.InitPhoneConfirmation(tx); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			context := &map[string]string{
				"site_url":                 config.SiteURL,
				"phone_confirmation_token": *user.PhoneConfirmationToken,
				"instance_url":             config.InstanceURL,
			}

			if err = sms.SendSMS(config.ConfirmationTemplate, user.Phone, context, config.SMS); err != nil {

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

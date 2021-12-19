package lib

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ResendEmail(db *gorm.DB, config *config.Config, e string) (bool, error) {

	user := &model.User{}

	now := time.Now()

	if tx := db.First(user, "email = ?", e); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return true, nil
		}
		logrus.Error(tx.Error)
		return true, handlers.ErrInternal
	}

	if !(config.DisableEmail || user.Email == nil || user.EmailConfirmed) {

		if user.EmailConfirmationTokenSentAt != nil && time.Since(*user.EmailConfirmationTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {
			return true, handlers.ErrTooManyRequests
		}

		token := randstr.String(100)

		user.EmailConfirmationToken = &token

		user.EmailConfirmationTokenSentAt = &now

		err := db.Transaction(func(tx *gorm.DB) error {

			if err := user.Save(db); err != nil {
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

			return nil

		})

		return true, err

	}

	return true, nil

}

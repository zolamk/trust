package anonymous

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ResendEmail(db *gorm.DB, config *config.Config, email string) (bool, error) {

	user := &model.User{}

	if err := db.First(user, "email = ?", email).Error; err != nil {

		if err == gorm.ErrRecordNotFound {

			return true, nil

		}

		logrus.Error(err)

		return true, handlers.ErrInternal

	}

	if user.EmailConfirmationTokenSentAt != nil && time.Since(*user.EmailConfirmationTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

		return true, handlers.ErrTooManyRequests

	}

	if !(config.DisableEmail || user.Email == nil || user.EmailConfirmed) {

		err := db.Transaction(func(tx *gorm.DB) error {

			if err := handlers.SendEmailConfirmation(user, tx, config); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			return nil

		})

		return true, err

	}

	return true, nil

}

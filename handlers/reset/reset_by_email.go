package reset

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ResetByEmail(db *gorm.DB, config *config.Config, email string, log_data *middleware.LogData) (bool, error) {

	user := &model.User{}

	if tx := db.First(user, "email = ?", email); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {

			return true, nil

		}

		logrus.Error(tx.Error)

		return false, handlers.ErrInternal

	}

	if user.RecoveryTokenSentAt != nil && time.Since(*user.RecoveryTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

		return true, nil

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		if user.EmailConfirmed {

			log := model.NewLog(user.ID, "reset by email initiated", log_data.IP, nil, log_data.UserAgent)

			if err := user.ResetByEmail(tx, log); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			context := map[string]string{
				"email":                email,
				"site_url":             config.SiteURL,
				"email_recovery_token": *user.RecoveryToken,
				"instance_url":         config.InstanceURL,
			}

			if user.Name != nil {

				context["name"] = *user.Name

			}

			if err := mail.SendEmail(config.RecoveryTemplate, context, email, config.SMTP); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		}

		return nil

	})

	return true, err

}

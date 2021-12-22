package reset

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Reset(db *gorm.DB, config *config.Config, username string, log_data *middleware.LogData) (bool, error) {

	user := &model.User{}

	if tx := db.First(user, "phone = ? or email = ?", username, username); tx.Error != nil {

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

		if user.Email != nil && *user.Email == username && user.EmailConfirmed {

			log := model.NewLog(user.ID, "reset by email initiated", log_data.IP, nil, log_data.Location, log_data.UserAgent)

			if err := user.ResetByEmail(tx, log); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			context := &map[string]string{
				"site_url":             config.SiteURL,
				"email_recovery_token": *user.RecoveryToken,
				"instance_url":         config.InstanceURL,
			}

			if err := mail.SendEmail(config.RecoveryTemplate, context, user.Email, config); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

		}

		if user.Phone != nil && *user.Phone == username && user.PhoneConfirmed {

			log := model.NewLog(user.ID, "reset by phone initiated", log_data.IP, nil, log_data.Location, log_data.UserAgent)

			if err := user.ResetByPhone(tx, log); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			context := &map[string]string{
				"site_url":             config.SiteURL,
				"phone_recovery_token": *user.RecoveryToken,
				"instance_url":         config.InstanceURL,
			}

			if err := sms.SendSMS(config.RecoveryTemplate, user.Phone, context, config.SMS); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

		}

		return nil

	})

	return true, err

}

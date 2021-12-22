package anonymous

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ResendPhone(db *gorm.DB, config *config.Config, p string) (bool, error) {

	user := &model.User{}

	now := time.Now()

	if tx := db.First(user, "phone = ?", p); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return true, nil
		}
		logrus.Error(tx.Error)
		return true, handlers.ErrInternal
	}

	if !(config.DisablePhone || user.Phone == nil || user.PhoneConfirmed) {

		if user.PhoneConfirmationTokenSentAt != nil && time.Since(*user.PhoneConfirmationTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {
			return true, handlers.ErrTooManyRequests
		}

		token := randstr.String(6)

		user.PhoneConfirmationToken = &token

		user.PhoneConfirmationTokenSentAt = &now

		err := db.Transaction(func(tx *gorm.DB) error {

			if err := user.Save(db); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			context := &map[string]string{
				"site_url":                 config.SiteURL,
				"phone_confirmation_token": *user.PhoneConfirmationToken,
				"instance_url":             config.InstanceURL,
			}

			if err := sms.SendSMS(config.ConfirmationTemplate, user.Phone, context, config.SMS); err != nil {
				return handlers.ErrInternal
			}

			return nil

		})

		return true, err

	}

	return true, nil

}

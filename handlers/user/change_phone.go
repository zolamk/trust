package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ChangePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, phone string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	err := db.Transaction(func(tx *gorm.DB) error {

		if !config.PhoneRule.MatchString(phone) {
			return handlers.ErrInvalidPhone
		}

		if tx := tx.First(user, "id = ?", token.Subject); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			return handlers.ErrInternal
		}

		if user.Phone != nil && *user.Phone == phone {
			return handlers.ErrNewPhoneSimilar
		}

		if tx := tx.First(user, "phone = ?", phone); tx.Error == nil {

			return handlers.ErrPhoneRegistered

		} else {

			if tx.Error != gorm.ErrRecordNotFound {

				logrus.Error(tx.Error)

				return handlers.ErrInternal

			}

		}

		if user.PhoneChangedAt != nil && time.Since(*user.PhoneChangedAt).Minutes() < float64(config.MinutesBetweenPhoneChange) {

			changable_at := user.PhoneChangedAt.Add(time.Minute * config.MinutesBetweenPhoneChange)

			err := handlers.ErrCantChangePhoneNow

			err.Extensions["changable_at"] = changable_at

			return err

		}

		if user.PhoneChangeTokenSentAt != nil && time.Since(*user.PhoneChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

			return handlers.ErrTooManyRequests

		}

		log := model.NewLog(user.ID, "phone change initiated", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err := user.ChangePhone(tx, log, phone); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"phone_change_token": *user.PhoneChangeToken,
			"new_phone":          *user.NewPhone,
			"instance_url":       config.InstanceURL,
		}

		if err := sms.SendSMS(config.ChangeTemplate, user.NewPhone, context, config.SMS); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		return nil

	})

	if err != nil {
		return nil, err
	}

	return user, nil

}

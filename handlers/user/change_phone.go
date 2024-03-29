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

	if !config.PhoneRule.MatchString(phone) {

		return nil, handlers.ErrInvalidPhone

	}

	if err := db.First(user, "id = ?", token.Subject).Error; err != nil {

		if err == gorm.ErrRecordNotFound {

			return nil, handlers.ErrUserNotFound

		}

		return nil, handlers.ErrInternal
	}

	if user.Phone != nil && *user.Phone == phone {

		return nil, handlers.ErrNewPhoneSimilar

	}

	if err := db.First(user, "phone = ?", phone).Error; err == nil {

		return nil, handlers.ErrPhoneRegistered

	} else if err != gorm.ErrRecordNotFound {

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		if user.PhoneChangedAt != nil && time.Since(*user.PhoneChangedAt).Minutes() < float64(config.MinutesBetweenPhoneChange) {

			changeable_at := user.PhoneChangedAt.Add(time.Minute * config.MinutesBetweenPhoneChange)

			err := handlers.ErrCantChangePhoneNow

			err.Extensions["changeable_at"] = changeable_at

			return err

		}

		if user.PhoneChangeTokenSentAt != nil && time.Since(*user.PhoneChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

			return handlers.ErrTooManyRequests

		}

		log := model.NewLog(user.ID, "phone change initiated", log_data.IP, nil, log_data.UserAgent)

		if err := user.ChangePhone(tx, log, phone); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		context := map[string]string{
			"site_url":           config.SiteURL,
			"phone_change_token": *user.PhoneChangeToken,
			"new_phone":          *user.NewPhone,
			"instance_url":       config.InstanceURL,
		}

		if user.Name != nil {

			context["name"] = *user.Name

		}

		if err := sms.SendSMS(config.ChangeTemplate, phone, context, config.SMS); err != nil {

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

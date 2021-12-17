package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ChangePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, phone string) (*model.User, error) {

	user := &model.User{}

	if !config.PhoneRule.MatchString(phone) {
		return nil, errors.ErrInvalidPhone
	}

	if tx := db.First(user, "id = ?", token.Subject); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, errors.ErrUserNotFound
		}
		return nil, errors.ErrInternal
	}

	if user.Phone != nil && *user.Phone == phone {
		return nil, errors.ErrNewPhoneSimilar
	}

	if tx := db.First(&model.User{}, "phone = ?", phone); tx.Error == nil {

		return nil, errors.ErrPhoneRegistered

	} else {

		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, errors.ErrInternal
		}

	}

	if config.AutoConfirm {

		user.Phone = &phone

		if err := user.Save(db); err != nil {
			return nil, errors.ErrInternal
		}

		return user, nil

	}

	if user.PhoneChangedAt != nil && time.Since(*user.PhoneChangedAt).Minutes() < float64(config.MinutesBetweenPhoneChange) {

		changable_at := user.PhoneChangedAt.Add(time.Minute * config.MinutesBetweenPhoneChange)

		err := errors.ErrCantChangePhoneNow

		err.Extensions["changable_at"] = changable_at

		return nil, err

	}

	if user.PhoneChangeTokenSentAt != nil && time.Since(*user.PhoneChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {
		return nil, errors.ErrTooManyRequests
	}

	change_token := randstr.String(6)

	now := time.Now()

	user.NewPhone = &phone

	user.PhoneChangeToken = &change_token

	user.PhoneChangeTokenSentAt = &now

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := user.Save(tx); err != nil {
			logrus.Error(err)
			return errors.ErrInternal
		}

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"phone_change_token": *user.PhoneChangeToken,
			"phone":              *user.Phone,
			"new_phone":          *user.NewPhone,
			"instance_url":       config.InstanceURL,
		}

		if err := sms.SendSMS(config.ChangeTemplate, user.NewPhone, context, config.SMS); err != nil {
			logrus.Error(err)
			return errors.ErrInternal
		}

		return nil

	})

	if err != nil {
		return nil, err
	}

	return user, nil

}

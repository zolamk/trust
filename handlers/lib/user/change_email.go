package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ChangeEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, new_email string) (*model.User, error) {

	user := &model.User{}

	if !config.EmailRule.MatchString(new_email) {
		return nil, handlers.ErrInvalidPhone
	}

	if tx := db.First(user, "id = ?", token.Subject); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}
		return nil, handlers.ErrInternal
	}

	if user.Email != nil && *user.Email == new_email {
		return nil, handlers.ErrNewEmailSimilar
	}

	if tx := db.First(&model.User{}, "email = ?", new_email); tx.Error == nil {

		return nil, handlers.ErrEmailRegistered

	} else {

		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, handlers.ErrInternal
		}

	}

	if user.EmailChangedAt != nil && time.Since(*user.EmailChangedAt).Minutes() < float64(config.MinutesBetweenEmailChange) {

		changable_at := user.EmailChangedAt.Add(time.Minute * config.MinutesBetweenEmailChange)

		err := handlers.ErrCantChangeEmailNow

		err.Extensions["changable_at"] = changable_at

		return nil, err

	}

	if config.AutoConfirm {

		user.Email = &new_email

		if err := user.Save(db); err != nil {
			return nil, handlers.ErrInternal
		}

		return user, nil

	}

	if user.EmailChangeTokenSentAt != nil && time.Since(*user.EmailChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {
		return nil, handlers.ErrTooManyRequests
	}

	change_token := randstr.String(100)

	now := time.Now()

	user.NewEmail = &new_email

	user.EmailChangeToken = &change_token

	user.EmailChangeTokenSentAt = &now

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := user.Save(tx); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"email_change_token": *user.EmailChangeToken,
			"email":              *user.Email,
			"new_email":          *user.NewEmail,
			"instance_url":       config.InstanceURL,
		}

		if err := email.SendEmail(config.ChangeTemplate, context, user.NewEmail, config); err != nil {
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

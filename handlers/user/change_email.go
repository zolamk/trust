package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ChangeEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, new_email string, log_data *middleware.LogData) (*model.User, error) {

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

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := db.First(user, "email = ?", new_email).Error; err == nil {

			return handlers.ErrEmailRegistered

		} else if err != gorm.ErrRecordNotFound {

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		if user.EmailChangedAt != nil && time.Since(*user.EmailChangedAt).Minutes() < float64(config.MinutesBetweenEmailChange) {

			changeable_at := user.EmailChangedAt.Add(time.Minute * config.MinutesBetweenEmailChange)

			err := handlers.ErrCantChangeEmailNow

			err.Extensions["changeable_at"] = changeable_at

			return err

		}

		if user.EmailChangeTokenSentAt != nil && time.Since(*user.EmailChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

			return handlers.ErrTooManyRequests

		}

		log := model.NewLog(user.ID, "email change initiated", log_data.IP, nil, log_data.UserAgent)

		if err := user.ChangeEmail(tx, log, new_email); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		context := map[string]string{
			"site_url":           config.SiteURL,
			"email_change_token": *user.EmailChangeToken,
			"new_email":          *user.NewEmail,
			"instance_url":       config.InstanceURL,
		}

		if user.Name != nil {

			context["name"] = *user.Name

		}

		if err := mail.SendEmail(config.ChangeTemplate, context, *user.NewEmail, config.SMTP); err != nil {
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

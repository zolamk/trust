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

	err := db.Transaction(func(tx *gorm.DB) error {

		if !config.EmailRule.MatchString(new_email) {

			return handlers.ErrInvalidPhone

		}

		if tx := db.First(user, "id = ?", token.Subject); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			return handlers.ErrInternal

		}

		if user.Email != nil && *user.Email == new_email {

			return handlers.ErrNewEmailSimilar

		}

		if tx := db.First(user, "email = ?", new_email); tx.Error == nil {

			return handlers.ErrEmailRegistered

		} else {

			if tx.Error != gorm.ErrRecordNotFound {

				logrus.Error(tx.Error)

				return handlers.ErrInternal

			}

		}

		if user.EmailChangedAt != nil && time.Since(*user.EmailChangedAt).Minutes() < float64(config.MinutesBetweenEmailChange) {

			changable_at := user.EmailChangedAt.Add(time.Minute * config.MinutesBetweenEmailChange)

			err := handlers.ErrCantChangeEmailNow

			err.Extensions["changable_at"] = changable_at

			return err

		}

		if user.EmailChangeTokenSentAt != nil && time.Since(*user.EmailChangeTokenSentAt).Minutes() < float64(config.MinutesBetweenResend) {

			return handlers.ErrTooManyRequests

		}

		log := model.NewLog(user.ID, "email change initiated", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err := user.ChangeEmail(tx, log, new_email); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"email_change_token": *user.EmailChangeToken,
			"new_email":          *user.NewEmail,
			"instance_url":       config.InstanceURL,
		}

		if err := mail.SendEmail(config.ChangeTemplate, context, user.NewEmail, config); err != nil {
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

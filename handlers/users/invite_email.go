package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func InviteEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, name string, email string, log_data *middleware.LogData) (*model.User, error) {

	if config.DisableEmail {
		return nil, handlers.ErrEmailDisabled
	}

	if !config.EmailRule.MatchString(email) {
		return nil, handlers.ErrInvalidEmail
	}

	is_admin := token.HasAdminRole()

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	user := &model.User{}

	err := db.Transaction(func(tx *gorm.DB) error {

		if tx := db.First(user, "email = ?", email); tx.Error == nil {

			return handlers.ErrEmailRegistered

		} else {

			if tx.Error != gorm.ErrRecordNotFound {

				logrus.Error(tx.Error)

				return handlers.ErrInternal

			}

		}

		if err := user.InviteByEmail(tx, name, email); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "invited by email", log_data.IP, &token.Subject, log_data.Location, log_data.UserAgent)

		if err := tx.Create(log).Error; err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		context := &map[string]string{
			"site_url":               config.SiteURL,
			"email_invitation_token": *user.EmailInvitationToken,
			"instance_url":           config.InstanceURL,
		}

		if err := mail.SendEmail(config.InvitationTemplate, context, user.Email, config); err != nil {

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

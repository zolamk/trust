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

	if err := handlers.ValidateEmail(email, db, config); err != nil {

		return nil, err

	}

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := user.InviteByEmail(tx, name, email); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "invited by email", log_data.IP, &token.Subject, log_data.UserAgent)

		if err := tx.Create(log).Error; err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		context := map[string]string{
			"site_url":               config.SiteURL,
			"email":                  *user.Email,
			"email_invitation_token": *user.EmailConfirmationToken,
			"instance_url":           config.InstanceURL,
		}

		if user.Name != nil {

			context["name"] = *user.Name

		}

		if err := mail.SendEmail(config.InvitationTemplate, context, *user.Email, config.SMTP); err != nil {

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

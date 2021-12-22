package users

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func InviteEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, name string, email string) (*model.User, error) {

	if config.DisableEmail {
		return nil, handlers.ErrEmailDisabled
	}

	if !config.EmailRule.MatchString(email) {
		return nil, handlers.ErrInvalidEmail
	}

	is_admin, err := token.IsAdmin(db)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if tx := db.First(&model.User{}, "email = ?", email); tx.Error == nil {
		return nil, handlers.ErrEmailRegistered
	} else {
		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, handlers.ErrInternal
		}
	}

	now := time.Now()

	invitation_token := randstr.String(100)

	user := &model.User{
		Name:                  &name,
		Email:                 &email,
		EmailInvitationToken:  &invitation_token,
		InvitationTokenSentAt: &now,
	}

	err = db.Transaction(func(tx *gorm.DB) error {

		if err := user.Save(tx); err != nil {
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

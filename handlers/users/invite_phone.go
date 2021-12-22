package users

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func InvitePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, name string, phone string) (*model.User, error) {

	if config.DisableEmail {
		return nil, handlers.ErrEmailDisabled
	}

	if !config.PhoneRule.MatchString(phone) {
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

	if tx := db.First(&model.User{}, "phone = ?", phone); tx.Error == nil {
		return nil, handlers.ErrPhoneRegistered
	} else {
		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, handlers.ErrInternal
		}
	}

	now := time.Now()

	invitation_token := randstr.String(6)

	user := &model.User{
		Name:                  &name,
		Phone:                 &phone,
		PhoneInvitationToken:  &invitation_token,
		InvitationTokenSentAt: &now,
	}

	err = db.Transaction(func(tx *gorm.DB) error {

		if err := user.Save(tx); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		context := &map[string]string{
			"site_url":               config.SiteURL,
			"phone_invitation_token": *user.PhoneInvitationToken,
			"instance_url":           config.InstanceURL,
		}

		if err := sms.SendSMS(config.InvitationTemplate, user.Phone, context, config.SMS); err != nil {
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

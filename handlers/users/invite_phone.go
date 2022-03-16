package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func InvitePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, name string, phone string, log_data *middleware.LogData) (*model.User, error) {

	if config.DisablePhone {
		return nil, handlers.ErrPhoneDisabled
	}

	if !config.PhoneRule.MatchString(phone) {
		return nil, handlers.ErrInvalidPhone
	}

	is_admin, err := token.HasAdminRole()

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	user := &model.User{}

	err = db.Transaction(func(tx *gorm.DB) error {

		if tx := db.First(user, "phone = ?", phone); tx.Error == nil {

			return handlers.ErrPhoneRegistered

		} else {

			if tx.Error != gorm.ErrRecordNotFound {

				logrus.Error(tx.Error)

				return handlers.ErrInternal

			}

		}

		if err := user.InviteByPhone(tx, name, phone); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "invited by phone", log_data.IP, &token.Subject, log_data.Location, log_data.UserAgent)

		if err := tx.Create(log).Error; err != nil {

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

package update

import (
	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func UpdatePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, phone string, confirm *bool) (*model.User, error) {

	is_admin, err := token.IsAdmin(db)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if token.Subject == id {
		return nil, handlers.ErrCantChangeOwnAccount
	}

	user := &model.User{}

	tx := db.First(user, "phone = ?", phone)

	if tx.Error != nil {

		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, handlers.ErrInternal
		}

	} else {

		err := handlers.ErrPhoneRegistered

		err.Extensions["email"] = user.Email

		err.Extensions["phone"] = user.Phone

		err.Extensions["id"] = user.ID

		return nil, err

	}

	if tx := db.Find(user, "id = ?", id); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}
		logrus.Error(tx.Error)
		return nil, handlers.ErrInternal
	}

	if !config.PhoneRule.MatchString(phone) {
		return nil, handlers.ErrInvalidPhone
	}

	db.Transaction(func(tx *gorm.DB) error {

		user.NewPhone = &phone

		if err := user.Save(db); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		if config.AutoConfirm || (confirm != nil && *confirm) {

			if err := user.ConfirmPhoneChange(tx); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			return nil

		}

		token := randstr.String(6)

		user.PhoneChangeToken = &token

		user.NewPhone = &phone

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"phone_change_token": *user.PhoneChangeToken,
			"new_phone":          *user.NewPhone,
			"instance_url":       config.InstanceURL,
		}

		if err := sms.SendSMS(config.ChangeTemplate, user.NewPhone, context, config.SMS); err != nil {
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

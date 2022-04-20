package update

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

func UpdatePhone(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, new_phone string, confirm bool, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	is_admin := token.HasAdminRole()

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if token.Subject == id {
		return nil, handlers.ErrCantChangeOwnAccount
	}

	err := db.Transaction(func(tx *gorm.DB) error {

		err := tx.First(user, "phone = ?", new_phone).Error

		if err != nil {

			if err != gorm.ErrRecordNotFound {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		} else {

			err := handlers.ErrPhoneRegistered

			err.Extensions["email"] = user.Email

			err.Extensions["phone"] = user.Phone

			err.Extensions["id"] = user.ID

			return err

		}

		if tx := db.Find(user, "id = ?", id); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		if !config.PhoneRule.MatchString(new_phone) {

			return handlers.ErrInvalidPhone

		}

		log := model.NewLog(user.ID, "phone change inititated by admin", log_data.IP, &token.Subject, log_data.UserAgent)

		if err := user.ChangePhone(tx, log, new_phone); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if confirm {

			log := model.NewLog(user.ID, "phone change confirmed by admin", log_data.IP, &token.Subject, log_data.UserAgent)

			if err := user.ConfirmPhoneChange(tx, log); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			return nil

		}

		context := map[string]string{
			"site_url":           config.SiteURL,
			"phone_change_token": *user.PhoneChangeToken,
			"new_phone":          new_phone,
			"instance_url":       config.InstanceURL,
		}

		if user.Name != nil {

			context["name"] = *user.Name

		}

		if err := sms.SendSMS(config.ChangeTemplate, new_phone, context, config.SMS); err != nil {

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

package update

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

func UpdateEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, new_email string, confirm bool, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	is_admin := token.HasAdminRole()

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if token.Subject == id {
		return nil, handlers.ErrCantChangeOwnAccount
	}

	err := db.Transaction(func(tx *gorm.DB) error {

		res := tx.First(user, "email = ?", new_email)

		if res.Error != nil {

			if res.Error != gorm.ErrRecordNotFound {

				logrus.Error(tx.Error)

				return handlers.ErrInternal

			}

		} else {

			err := handlers.ErrEmailRegistered

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

		if !config.EmailRule.MatchString(new_email) {

			return handlers.ErrInvalidEmail

		}

		log := model.NewLog(user.ID, "email change inititated by admin", log_data.IP, &token.Subject, log_data.UserAgent)

		if err := user.ChangeEmail(tx, log, new_email); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if confirm {

			log := model.NewLog(user.ID, "email change confirmed by admin", log_data.IP, &token.Subject, log_data.UserAgent)

			if err := user.ConfirmEmailChange(tx, log); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			return nil

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

		if err := mail.SendEmail(config.ChangeTemplate, context, new_email, config.SMTP); err != nil {

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

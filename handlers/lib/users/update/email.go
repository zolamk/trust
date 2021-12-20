package update

import (
	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/email"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func UpdateEmail(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, new_email string, confirm *bool) (*model.User, error) {

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

	tx := db.First(user, "email = ?", new_email)

	if tx.Error != nil {

		if tx.Error != gorm.ErrRecordNotFound {
			logrus.Error(tx.Error)
			return nil, handlers.ErrInternal
		}

	} else {

		err := handlers.ErrEmailRegistered

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

	if !config.EmailRule.MatchString(new_email) {
		return nil, handlers.ErrInvalidEmail
	}

	err = db.Transaction(func(tx *gorm.DB) error {

		user.NewEmail = &new_email

		if err := user.Save(db); err != nil {
			logrus.Error(err)
			return handlers.ErrInternal
		}

		if config.AutoConfirm || (confirm != nil && *confirm) {

			if err := user.ConfirmEmailChange(tx); err != nil {
				logrus.Error(err)
				return handlers.ErrInternal
			}

			return nil

		}

		token := randstr.String(100)

		user.EmailChangeToken = &token

		user.NewEmail = &new_email

		context := &map[string]string{
			"site_url":           config.SiteURL,
			"email_change_token": *user.EmailChangeToken,
			"new_email":          *user.NewEmail,
			"instance_url":       config.InstanceURL,
		}

		if err := email.SendEmail(config.ChangeTemplate, context, user.NewEmail, config); err != nil {
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

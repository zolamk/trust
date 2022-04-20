package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func AcceptEmailInvite(db *gorm.DB, c *config.Config, token string, password string, log_data *middleware.LogData) (*model.User, error) {

	user := model.NewUser(c)

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := tx.First(user, "email_confirmation_token = ?", token).Error; err != nil {

			if err == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if err := user.SetPassword(password, int(c.PasswordHashCost)); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "accepted email invitation", log_data.IP, nil, log_data.UserAgent)

		if err := user.AcceptEmailInvite(tx, log); err != nil {

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

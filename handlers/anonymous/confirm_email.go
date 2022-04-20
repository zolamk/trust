package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmEmail(db *gorm.DB, config *config.Config, token string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	err := db.Transaction(func(tx *gorm.DB) error {

		if err := tx.First(user, "email_confirmation_token = ?", token).Error; err != nil {

			if err == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "email confirmed", log_data.IP, nil, log_data.UserAgent)

		if err := user.ConfirmEmail(tx, log); err != nil {

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

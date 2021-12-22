package anonymous

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmPhone(db *gorm.DB, config *config.Config, token string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	db.Transaction(func(tx *gorm.DB) error {

		if tx := tx.First(user, "phone_confirmation_token = ?", token); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "phone confirmed", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err := user.ConfirmPhone(tx, log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		return nil

	})
	return user, nil

}

package user

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmPhoneChange(db *gorm.DB, config *config.Config, token *jwt.JWT, phone_change_token string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	err := db.Transaction(func(tx *gorm.DB) error {

		if tx := tx.First(user, "id = ? AND phone_change_token = ?", token.Subject, phone_change_token); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {
				return handlers.ErrUserNotFound
			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "phone change confirmed", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err := user.ConfirmPhoneChange(tx, log); err != nil {

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

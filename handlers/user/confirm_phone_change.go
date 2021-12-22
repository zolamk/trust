package user

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmPhoneChange(db *gorm.DB, config *config.Config, token *jwt.JWT, phone_change_token string) (*model.User, error) {

	user := &model.User{}

	if tx := db.First(user, "id = ? AND phone_change_token = ?", token.Subject, phone_change_token); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	if err := user.ConfirmPhoneChange(db); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	return user, nil

}
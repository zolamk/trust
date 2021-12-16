package lib

import (
	"errors"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmPhone(db *gorm.DB, config *config.Config, token string) (*model.User, error) {

	internal_error := errors.New("internal server error")

	user := &model.User{}

	if tx := db.First(user, "phone_confirmation_token = ?", token); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, errors.New("user not found")
		}
		logrus.Error(tx.Error)
		return nil, internal_error
	}

	if err := user.ConfirmPhone(db); err != nil {
		logrus.Error(err)
		return nil, internal_error
	}

	return user, nil

}

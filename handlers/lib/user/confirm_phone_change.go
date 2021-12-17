package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmPhoneChange(db *gorm.DB, config *config.Config, token *jwt.JWT, phone_change_token string) (*model.User, error) {

	user := &model.User{}

	if tx := db.First(user, "id = ? AND phone_change_token = ?", token.Subject, phone_change_token); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, errors.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, errors.ErrInternal

	}

	now := time.Now()

	user.Phone = user.NewPhone

	user.NewPhone = nil

	user.PhoneChangedAt = &now

	user.PhoneChangeToken = nil

	user.PhoneChangeTokenSentAt = nil

	if err := user.Save(db); err != nil {
		logrus.Error(err)
		return nil, errors.ErrInternal
	}

	return user, nil

}

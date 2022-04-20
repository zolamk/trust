package user

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Me(db *gorm.DB, config *config.Config, token *jwt.JWT) (*model.User, error) {

	user := &model.User{}

	if tx := db.First(user, "id = ?", token.Subject); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {

			return nil, handlers.ErrUserNotFound

		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	return user, nil

}

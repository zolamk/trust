package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func User(db *gorm.DB, config *config.Config, token *jwt.JWT, id string) (*model.User, error) {

	user := &model.User{}

	if config.AdminOnlyList && id != token.Subject {

		is_admin, err := token.IsAdmin(db)

		if err != nil {
			logrus.Error(err)
			return nil, handlers.ErrInternal
		}

		if !is_admin {
			return nil, handlers.ErrAdminOnly
		}

	}

	if tx := db.First(user, "id = ?", id); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	return user, nil

}

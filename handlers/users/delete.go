package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func DeleteUser(db *gorm.DB, config *config.Config, token *jwt.JWT, id string) (*model.User, error) {

	is_admin := token.HasAdminRole()

	if !is_admin {

		return nil, handlers.ErrAdminOnly

	}

	if token.Subject == id {

		return nil, handlers.ErrCantChangeOwnAccount

	}

	user := &model.User{}

	if err := db.First(user, "id = ?", id).Error; err != nil {

		if err == gorm.ErrRecordNotFound {

			return nil, handlers.ErrUserNotFound

		}

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	if err := db.Delete(user, "id = ?", id); err != nil {

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	return user, nil
}

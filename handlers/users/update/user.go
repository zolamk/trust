package update

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func UpdateUser(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, name *string, avatar *string) (*model.User, error) {

	is_admin := token.HasAdminRole()

	if !is_admin {

		return nil, handlers.ErrAdminOnly

	}

	if token.Subject == id {

		return nil, handlers.ErrCantChangeOwnAccount

	}

	user := &model.User{}

	if tx := db.Find(user, "id = ?", id); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {

			return nil, handlers.ErrUserNotFound

		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	user.Name = name

	user.Avatar = avatar

	if err := user.Save(db); err != nil {

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	return user, nil

}

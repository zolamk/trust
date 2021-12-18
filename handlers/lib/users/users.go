package users

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Users(db *gorm.DB, config *config.Config, token *jwt.JWT, limit, offset int) ([]*model.User, error) {

	users := &[]*model.User{}

	if config.AdminOnlyList {

		is_admin, err := token.IsAdmin(db)

		if err != nil {
			logrus.Error(err)
			return *users, errors.ErrInternal
		}

		if !is_admin {
			return *users, errors.ErrAdminOnly
		}

	}

	if tx := db.Limit(limit).Offset(offset).Find(users); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return *users, nil
		}

		logrus.Error(tx.Error)

		return nil, errors.ErrInternal

	}

	return *users, nil

}

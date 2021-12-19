package users

import (
	"log"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/compilers"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Users(db *gorm.DB, config *config.Config, token *jwt.JWT, fields []string, where map[string]interface{}, order_by map[string]interface{}, offset, limit int) ([]*model.User, error) {

	users := []*model.User{}

	if config.AdminOnlyList {

		is_admin, err := token.IsAdmin(db)

		if err != nil {
			logrus.Error(err)
			return users, handlers.ErrInternal
		}

		if !is_admin {
			return users, handlers.ErrAdminOnly
		}

	}

	query, params, err := compilers.CompileQuery(fields, where, order_by, offset, limit)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	log.Println(*query)

	if tx := db.Raw(*query, params...).Scan(&users); tx.Error != nil {
		logrus.Error(tx.Error)
		return users, handlers.ErrInternal
	}

	return users, nil

}

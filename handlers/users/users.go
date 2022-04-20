package users

import (
	"log"

	"github.com/jackc/pgconn"
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/compilers"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Users(db *gorm.DB, config *config.Config, token *jwt.JWT, fields []string, where map[string]interface{}, order_by []model.Object, offset, limit int) ([]*model.User, error) {

	users := []*model.User{}

	if config.AdminOnlyList {

		has_access := token.HasAdminRole() || token.HasReadRole()

		if !has_access {

			return users, handlers.ErrAdminOnly

		}

	}

	query, params, err := compilers.CompileQuery(fields, where, order_by, offset, limit)

	if err != nil {

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	logrus.WithField("params", params).Debug(*query)

	if tx := db.Raw(*query, params...).Scan(&users); tx.Error != nil {

		logrus.Error(tx.Error)

		if pe, ok := tx.Error.(*pgconn.PgError); ok {

			log.Println(pe.Code)

			err := handlers.ErrDataException

			switch pe.Code {

			case "22007", "22008", "22P02":

				err.Message = pe.Message

				return users, err

			}

		}

		return users, handlers.ErrInternal

	}

	return users, nil

}

package users

import (
	"github.com/doug-martin/goqu/v9"
	"github.com/jackc/pgconn"
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/compilers"
	"gorm.io/gorm"
)

func UsersCount(db *gorm.DB, config *config.Config, token *jwt.JWT, where map[string]interface{}) (int, error) {

	if config.AdminOnlyList {

		has_access := token.HasAdminRole() || token.HasReadRole()

		if !has_access {

			return 0, handlers.ErrAdminOnly

		}

	}

	var count int

	complied_where := compilers.CompileWhere(where)

	query, params, err := goqu.From("trust.users").Prepared(true).Select(goqu.COUNT("*")).Where(complied_where).ToSQL()

	if err != nil {

		return 0, handlers.ErrInternal

	}

	logrus.WithField("params", params).Debug(query)

	if err := db.Raw(query, params...).Scan(&count).Error; err != nil {

		logrus.Error(err)

		if pe, ok := err.(*pgconn.PgError); ok {

			err := handlers.ErrDataException

			switch pe.Code {

			case "22007", "22008", "22P02":

				err.Message = pe.Message

				return 0, err

			}

		}

		return 0, handlers.ErrInternal

	}

	return count, nil

}

package users

import (
	"log"

	"github.com/99designs/gqlgen/graphql"
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/graphjin/psql"
	"github.com/zolamk/trust/graphjin/qcode"
	"github.com/zolamk/trust/graphjin/sdata"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Users(db *gorm.DB, config *config.Config, token *jwt.JWT, oc *graphql.OperationContext, db_schema *sdata.DBSchema) ([]*model.User, error) {

	users := &[]*model.User{}

	// if config.AdminOnlyList {

	// 	is_admin, err := token.IsAdmin(db)

	// 	if err != nil {
	// 		logrus.Error(err)
	// 		return *users, errors.ErrInternal
	// 	}

	// 	if !is_admin {
	// 		return *users, errors.ErrAdminOnly
	// 	}

	// }

	qcode_compiler, err := qcode.NewCompiler(db_schema, qcode.Config{
		DBSchema:     "trust",
		DisableAgg:   true,
		DisableFuncs: true,
	})

	if err != nil {
		logrus.Error(err)
		return *users, errors.ErrInternal
	}

	qcode, err := qcode_compiler.Compile([]byte(oc.RawQuery), qcode.Variables{}, "user")

	if err != nil {
		logrus.Error(err)
		return *users, errors.ErrInternal
	}

	psql_compiler := psql.NewCompiler(psql.Config{
		Vars: map[string]string{},
	})

	_, query, err := psql_compiler.CompileEx(qcode)

	if err != nil {
		logrus.Error(err)
		return *users, errors.ErrInternal
	}

	log.Println(string(query))

	if tx := db.Raw(string(query)).Scan(users); tx.Error != nil {
		logrus.Error(tx.Error)
		return *users, errors.ErrInternal
	}

	return *users, nil

}

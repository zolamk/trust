package user

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Logs(db *gorm.DB, config *config.Config, token *jwt.JWT, offset, limit int) ([]*model.Log, error) {

	logs := []*model.Log{}

	if tx := db.Find(&logs, "user_id = ?", token.Subject).Offset(offset).Limit(limit); tx.Error != nil {
		logrus.Error(tx.Error)
		return logs, handlers.ErrInternal
	}

	return logs, nil

}

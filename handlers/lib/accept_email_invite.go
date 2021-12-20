package lib

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func AcceptEmailInvite(db *gorm.DB, c *config.Config, token string, password string) (*model.User, error) {

	user := &model.User{}

	if tx := db.First(user, "email_invitation_token = ?", token); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	user.SetPassword(password, int(c.PasswordHashCost))

	if err := user.AcceptEmailInvite(db); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	return user, nil

}

package update

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func UpdatePassword(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, password string, log_data *middleware.LogData) (*model.User, error) {

	is_admin, err := token.HasAdminRole()

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if !is_admin {
		return nil, handlers.ErrAdminOnly
	}

	if token.Subject == id {
		return nil, handlers.ErrCantChangeOwnAccount
	}

	if !config.PasswordRule.MatchString(password) {
		return nil, handlers.ErrInvalidPassword
	}

	user := &model.User{}

	err = db.Transaction(func(tx *gorm.DB) error {

		if tx := db.Find(user, "id = ?", id); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrUserNotFound

			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		user.SetPassword(password, int(config.PasswordHashCost))

		now := time.Now()

		user.PasswordChangedAt = &now

		log := model.NewLog(user.ID, "password changed by admin", log_data.IP, &token.Subject, log_data.Location, log_data.UserAgent)

		if err := user.ChangePassword(tx, log, password, int(config.PasswordHashCost)); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		return nil

	})

	if err != nil {
		return nil, err
	}

	return user, nil

}

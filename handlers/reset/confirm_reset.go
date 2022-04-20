package reset

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func ConfirmReset(db *gorm.DB, config *config.Config, recovery_token string, pwd string, log_data *middleware.LogData) (bool, error) {

	user := &model.User{}

	err := db.Transaction(func(tx *gorm.DB) error {

		if !config.PasswordRule.MatchString(pwd) {

			return handlers.ErrInvalidPassword

		}

		if tx := db.First(user, "recovery_token = ?", recovery_token); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {

				return handlers.ErrRecoveryTokenNotFound

			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		if err := user.SetPassword(pwd, int(config.PasswordHashCost)); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "password reset confirmed", log_data.IP, nil, log_data.UserAgent)

		if err := user.ConfirmReset(tx, log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		return nil

	})

	if err != nil {
		return false, err
	}

	return true, nil

}

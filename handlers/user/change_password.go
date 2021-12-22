package user

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func ChangePassword(db *gorm.DB, config *config.Config, token *jwt.JWT, old_password, new_password string, log_data *middleware.LogData) (*model.User, error) {

	user := &model.User{}

	if !config.PasswordRule.MatchString(new_password) {
		return nil, handlers.ErrInvalidPassword
	}

	err := db.Transaction(func(tx *gorm.DB) error {

		if tx := tx.First(user, "id = ?", token.Subject); tx.Error != nil {

			if tx.Error == gorm.ErrRecordNotFound {
				return handlers.ErrUserNotFound
			}

			logrus.Error(tx.Error)

			return handlers.ErrInternal

		}

		if err := user.VerifyPassword(old_password); err != nil {

			if err == bcrypt.ErrMismatchedHashAndPassword {

				return handlers.ErrIncorrectOldPassword

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		log := model.NewLog(user.ID, "password changed", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err := user.ChangePassword(tx, log, new_password, int(config.PasswordHashCost)); err != nil {

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

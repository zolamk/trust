package reset

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func ConfirmReset(db *gorm.DB, config *config.Config, recovery_token string, pwd string) (bool, error) {

	user := &model.User{}

	if !config.PasswordRule.MatchString(pwd) {
		return false, handlers.ErrInvalidPassword
	}

	if tx := db.First(user, "recovery_token = ?", recovery_token); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return false, handlers.ErrRecoveryTokenNotFound
		}
		logrus.Error(tx.Error)
		return false, handlers.ErrInternal
	}

	password, err := bcrypt.GenerateFromPassword([]byte(pwd), int(config.PasswordHashCost))

	if err != nil {
		logrus.Error(err)
		return false, handlers.ErrInternal
	}

	hash := string(password)

	user.Password = &hash

	user.RecoveryToken = nil

	user.RecoveryTokenSentAt = nil

	if err := user.Save(db); err != nil {
		logrus.Error(err)
		return false, handlers.ErrInternal
	}

	return true, nil

}

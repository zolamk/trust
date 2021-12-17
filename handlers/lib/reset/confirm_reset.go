package reset

import (
	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func ConfirmReset(db *gorm.DB, config *config.Config, recovery_token string, pwd string) (bool, error) {

	user := &model.User{}

	if tx := db.First(user, "recovery_token = ?", recovery_token); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return false, errors.RecoveryTokenNotFound
		}
		logrus.Error(tx.Error)
		return false, errors.Internal
	}

	password, err := bcrypt.GenerateFromPassword([]byte(pwd), int(config.PasswordHashCost))

	if err != nil {
		logrus.Error(err)
		return false, errors.Internal
	}

	hash := string(password)

	user.Password = &hash

	user.RecoveryToken = nil

	user.RecoveryTokenSentAt = nil

	if err := user.Save(db); err != nil {
		logrus.Error(err)
		return false, errors.Internal
	}

	return true, nil

}

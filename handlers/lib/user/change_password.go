package user

import (
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func ChangePassword(db *gorm.DB, config *config.Config, token *jwt.JWT, old_password, new_password string) (*model.User, error) {

	user := &model.User{}

	if !config.PasswordRule.MatchString(new_password) {
		return nil, handlers.ErrInvalidPassword
	}

	if tx := db.First(user, "id = ?", token.Subject); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	if err := bcrypt.CompareHashAndPassword([]byte(*user.Password), []byte(old_password)); err != nil {

		if err == bcrypt.ErrMismatchedHashAndPassword {
			return nil, handlers.ErrIncorrectOldPassword
		}

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	password, err := bcrypt.GenerateFromPassword([]byte(new_password), int(config.PasswordHashCost))

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	hash := string(password)

	now := time.Now()

	user.Password = &hash

	user.PasswordChangedAt = &now

	if err := user.Save(db); err != nil {

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	return user, nil

}

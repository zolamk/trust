package update

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

func UpdatePassword(db *gorm.DB, config *config.Config, token *jwt.JWT, id string, password string) (*model.User, error) {

	is_admin, err := token.IsAdmin(db)

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

	user := &model.User{}

	if tx := db.Find(user, "id = ?", id); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrUserNotFound
		}
		logrus.Error(tx.Error)
		return nil, handlers.ErrInternal
	}

	if !config.PasswordRule.MatchString(password) {
		return nil, handlers.ErrInvalidPassword
	}

	pwd, err := bcrypt.GenerateFromPassword([]byte(password), int(config.PasswordHashCost))

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	hash := string(pwd)

	now := time.Now()

	user.PasswordChangedAt = &now

	user.Password = &hash

	if err := user.Save(db); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	return user, nil

}

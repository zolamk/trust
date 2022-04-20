package anonymous

import (
	"net/http"
	"time"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func Token(db *gorm.DB, config *config.Config, username string, password string, writer http.ResponseWriter, log_data *middleware.LogData) (*model.LoginResponse, error) {

	user := model.NewUser(config)

	var signed_token string

	var refresh_token string

	var err error

	err = db.Transaction(func(tx *gorm.DB) error {

		if err = tx.First(user, "phone = ? or email = ?", username, username).Error; err != nil {

			if err == gorm.ErrRecordNotFound {

				return handlers.ErrIncorrectUsernameOrPassword

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		if user.Email != nil && *user.Email == username && !user.EmailConfirmed {

			return handlers.ErrEmailNotConfirmed

		}

		if user.Phone != nil && *user.Phone == username && !user.PhoneConfirmed {

			return handlers.ErrPhoneNotConfirmed

		}

		if err := checkLoginAttempt(user, tx); err != nil {

			return err

		}

		if err = verifyPassword(password, log_data.IP, log_data.UserAgent, user, tx, config); err != nil {

			return err

		}

		signed_token, refresh_token, err = handlers.SignIn(user, log_data.IP, log_data.UserAgent, tx, writer)

		if err != nil {

			return err

		}

		return nil

	})

	if err != nil {

		return nil, err

	}

	return &model.LoginResponse{
		AccessToken:  signed_token,
		RefreshToken: refresh_token,
		ID:           user.ID,
	}, nil

}

func verifyPassword(password, ip, ua string, user *model.User, tx *gorm.DB, config *config.Config) error {

	if err := user.VerifyPassword(password); err != nil {

		if err == bcrypt.ErrMismatchedHashAndPassword {

			log := model.NewLog(user.ID, "incorrect login attempt", ip, nil, ua)

			if err = user.IncorrectAttempt(tx, log); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

			err := handlers.ErrIncorrectUsernameOrPassword

			err.Extensions["password_set"] = user.Password != nil

			err.Extensions["remaining_attempts"] = int(config.LockoutPolicy.Attempts) - user.IncorrectLoginAttempts

			return handlers.ErrIncorrectUsernameOrPassword

		}

		logrus.Error(err)

		return handlers.ErrInternal

	}

	return nil

}

func checkLoginAttempt(user *model.User, tx *gorm.DB) error {

	if user.FinishedLoginAttempts() {

		now := time.Now()

		unlocked_at := user.AccountUnlockedAt()

		if now.Before(unlocked_at) {

			err := handlers.ErrAccountLocked

			err.Extensions["unlocked_at"] = unlocked_at

			return handlers.ErrAccountLocked

		}

		if err := user.ResetAttempt(tx); err != nil {

			logrus.Errorln(err)

			return handlers.ErrInternal

		}

	}

	return nil

}

package anonymous

import (
	"net/http"
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func Token(db *gorm.DB, config *config.Config, username string, password string, writer http.ResponseWriter, log_data *middleware.LogData) (*model.LoginResponse, error) {

	user := &model.User{}

	var signed_token string

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

		if user.IncorrectLoginAttempts >= config.LockoutPolicy.Attempts {

			now := time.Now()

			unlocked_at := user.LastIncorrectLoginAttemptAt.Add(time.Minute * config.LockoutPolicy.For)

			if now.Before(unlocked_at) {

				err := handlers.ErrAccountLocked

				err.Extensions["unlocked_at"] = unlocked_at

				return handlers.ErrAccountLocked

			}

			if err = user.ResetAttempt(db); err != nil {

				logrus.Error(err)

				return handlers.ErrInternal

			}

		}

		if err = user.VerifyPassword(password); err != nil {

			if err == bcrypt.ErrMismatchedHashAndPassword {

				log := model.NewLog(user.ID, "incorrect login", log_data.IP, nil, log_data.Location, log_data.UserAgent)

				if err = user.IncorrectAttempt(db, log); err != nil {

					logrus.Error(err)

					return handlers.ErrInternal

				}

				err := handlers.ErrIncorrectUsernameOrPassword

				err.Extensions["password_set"] = user.Password != nil

				err.Extensions["remaining_attempts"] = config.LockoutPolicy.Attempts - user.IncorrectLoginAttempts

				return handlers.ErrIncorrectUsernameOrPassword

			}

			logrus.Error(err)

			return handlers.ErrInternal

		}

		hook_user := *user

		if !hook_user.EmailConfirmed {
			hook_user.Email = nil
		}

		if !hook_user.PhoneConfirmed {
			hook_user.Phone = nil
		}

		payload := &map[string]interface{}{
			"event":    "login",
			"provider": "password",
			"user":     hook_user,
		}

		hook_response, err := hook.TriggerHook("login", payload, config)

		if err != nil {

			logrus.Error(err)

			return handlers.ErrWebHook

		}

		token := jwt.New("password", user, hook_response, config.JWT)

		signed_token, err = token.Sign()

		if err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		refresh_token := model.RefreshToken{
			Token:  randstr.String(50),
			UserID: user.ID,
		}

		if err = refresh_token.Create(tx); err != nil {

			logrus.Error(err)

			return nil

		}

		log := model.NewLog(user.ID, "login", log_data.IP, nil, log_data.Location, log_data.UserAgent)

		if err = user.SignedIn(tx, log); err != nil {

			logrus.Error(err)

			return handlers.ErrInternal

		}

		cookie := &http.Cookie{
			HttpOnly: true,
			Secure:   true,
			Name:     config.RefreshTokenCookieName,
			SameSite: http.SameSiteStrictMode,
			Value:    refresh_token.Token,
		}

		http.SetCookie(writer, cookie)

		cookie = &http.Cookie{
			HttpOnly: true,
			Secure:   true,
			Name:     config.AccessTokenCookieName,
			SameSite: http.SameSiteStrictMode,
			Value:    signed_token,
			Expires:  token.ExpiresAt.Time,
			Domain:   config.AccessTokenCookieDomain,
		}

		http.SetCookie(writer, cookie)

		return nil

	})

	if err != nil {
		return nil, err
	}

	return &model.LoginResponse{
		AccessToken: signed_token,
		ID:          user.ID,
	}, nil

}

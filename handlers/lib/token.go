package lib

import (
	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

func Token(db *gorm.DB, config *config.Config, username string, password string) (*model.LoginResponse, error) {

	user := &model.User{}

	if tx := db.First(user, "phone = ? or email = ?", username, username); tx.Error != nil {

		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrIncorrectUsernameOrPassword
		}

		logrus.Error(tx.Error)

		return nil, handlers.ErrInternal

	}

	if user.Email != nil && *user.Email == username && !user.EmailConfirmed {
		return nil, handlers.ErrEmailNotConfirmed
	}

	if user.Phone != nil && *user.Phone == username && !user.PhoneConfirmed {
		return nil, handlers.ErrPhoneNotConfirmed
	}

	if user.Password == nil {
		return nil, handlers.ErrIncorrectUsernameOrPassword
	}

	if err := bcrypt.CompareHashAndPassword([]byte(*user.Password), []byte(password)); err != nil {

		if err == bcrypt.ErrMismatchedHashAndPassword {
			return nil, handlers.ErrIncorrectUsernameOrPassword
		}

		logrus.Error(err)

		return nil, handlers.ErrInternal

	}

	payload := &map[string]interface{}{
		"event":    "login",
		"provider": "email",
		"user":     user,
	}

	hook_response, err := hook.TriggerHook("login", payload, config)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrWebHook
	}

	token := jwt.New(user, hook_response, config.JWT)

	signed_token, err := token.Sign()

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	refresh_token := model.RefreshToken{
		Token:  randstr.String(50),
		UserID: user.ID,
	}

	if err := refresh_token.Create(db); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	return &model.LoginResponse{
		AccessToken:  signed_token,
		RefreshToken: refresh_token.Token,
		ID:           user.ID,
	}, nil

}

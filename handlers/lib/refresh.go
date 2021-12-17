package lib

import (
	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func RefreshToken(db *gorm.DB, config *config.Config, rt string) (*model.LoginResponse, error) {

	var refresh_token model.RefreshToken

	if tx := db.Joins("User").First(&refresh_token, "token = ?", rt); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, errors.RefreshTokenNotFound
		}
		return nil, errors.Internal
	}

	user := refresh_token.User

	refresh_token.Token = randstr.String(50)

	payload := &map[string]interface{}{
		"event":    "login",
		"provider": "email",
		"user":     user,
	}

	hook_response, err := hook.TriggerHook("login", payload, config)

	if err != nil {
		logrus.Error(err)
		return nil, errors.WebHook
	}

	token := jwt.New(user, hook_response, config.JWT)

	signed_token, err := token.Sign()

	if err != nil {
		logrus.Error(err)
		return nil, errors.Internal
	}

	if err := refresh_token.Save(db); err != nil {
		logrus.Error(err)
		return nil, errors.Internal
	}

	return &model.LoginResponse{
		AccessToken:  signed_token,
		RefreshToken: refresh_token.Token,
		ID:           user.ID,
	}, nil
}

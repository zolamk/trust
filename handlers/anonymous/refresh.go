package anonymous

import (
	"net/http"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func RefreshToken(db *gorm.DB, config *config.Config, rt string, provider string, writer http.ResponseWriter) (*model.LoginResponse, error) {

	var refresh_token model.RefreshToken

	if tx := db.Joins("User").First(&refresh_token, "token = ?", rt); tx.Error != nil {
		if tx.Error == gorm.ErrRecordNotFound {
			return nil, handlers.ErrRefreshTokenNotFound
		}
		return nil, handlers.ErrInternal
	}

	user := refresh_token.User

	refresh_token.Token = randstr.String(50)

	hook_user := *user

	if !hook_user.EmailConfirmed {
		hook_user.Email = nil
	}

	if !hook_user.PhoneConfirmed {
		hook_user.Phone = nil
	}

	payload := &map[string]interface{}{
		"event":    "login",
		"provider": provider,
		"user":     hook_user,
	}

	hook_response, err := hook.TriggerHook(hook_user.ID, "login", payload, config)

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrWebHook
	}

	token := jwt.New(provider, user, hook_response, config)

	signed_token, err := token.Sign()

	if err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	if err := refresh_token.Save(db); err != nil {
		logrus.Error(err)
		return nil, handlers.ErrInternal
	}

	cookie := &http.Cookie{
		HttpOnly: true,
		Name:     config.RefreshTokenCookieName,
		Value:    refresh_token.Token,
	}

	http.SetCookie(writer, cookie)

	return &model.LoginResponse{
		AccessToken: signed_token,
		ID:          user.ID,
	}, nil

}

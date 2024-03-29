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
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func RefreshToken(db *gorm.DB, config *config.Config, rt interface{}, pr interface{}, writer http.ResponseWriter) (*model.LoginResponse, error) {

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

	provider := "password"

	if pr != nil {
		provider = pr.(string)
	}

	payload := &map[string]interface{}{
		"event":    "login",
		"provider": provider,
		"user":     hook_user,
	}

	hook_response, err := hook.TriggerHook(hook_user.ID, "login", payload, config)

	if err != nil {

		logrus.Error(err)

		e := handlers.ErrWebHook

		e.Message = err.Error()

		return nil, e

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

	if config.SetRefreshTokenCookie {

		cookie := &http.Cookie{
			HttpOnly: true,
			Secure:   true,
			Path:     "/",
			Name:     config.RefreshTokenCookieName,
			SameSite: http.SameSiteStrictMode,
			Value:    refresh_token.Token,
			Expires:  time.Now().Add(time.Hour * 24 * 7),
			Domain:   config.RefreshTokenCookieDomain,
		}

		http.SetCookie(writer, cookie)

	}

	return &model.LoginResponse{
		AccessToken:  signed_token,
		RefreshToken: refresh_token.Token,
		ID:           user.ID,
	}, nil

}

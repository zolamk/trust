package provider

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func Callback(db *gorm.DB, config *config.Config) http.HandlerFunc {

	return func(res http.ResponseWriter, req *http.Request) {

		log_data := req.Context().Value(middleware.LogDataKey).(middleware.LogData)

		internal_redirect := fmt.Sprintf("%s/%s?error=internal_error", config.SiteURL, config.SocialRedirectPage)

		code := req.URL.Query().Get("code")

		state, err := verify(req.URL.Query().Get("state"), config)

		if err != nil {

			logrus.Error(err)

			redirect_url := fmt.Sprintf("%s/%s?error=invalid_state", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		oauth_provider, err := get_provider(state.Provider, config)

		if err != nil {

			redirect_url := fmt.Sprintf("%s/%s?error=unknown_provider", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		if !oauth_provider.enabled() {

			provider_disabled := fmt.Sprintf("%s/%s?error=provider_disabled", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, provider_disabled, http.StatusTemporaryRedirect)

			return

		}

		provider_config := oauth_provider.get_config()

		token, err := provider_config.Exchange(context.Background(), code)

		if err != nil {

			redirect_url := fmt.Sprintf("%s/%s?error=error_exchanging_code", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		user_data, err := oauth_provider.get_user_data(token.AccessToken)

		if err != nil {

			logrus.Error(err)

			redirect_url := fmt.Sprintf("%s/%s?error=error_getting_user_data", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		if user_data.Email == nil {

			redirect_url := fmt.Sprintf("%s/%s?error=email_not_found", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		if !user_data.Verified {

			redirect_url := fmt.Sprintf("%s/%s?error=email_not_verified", config.SiteURL, config.SocialRedirectPage)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return

		}

		user := &model.User{}

		err = db.Transaction(func(tx *gorm.DB) error {

			if tx := tx.First(user, "email = ?", user_data.Email); tx.Error != nil {

				if tx.Error != gorm.ErrRecordNotFound {

					logrus.Error(tx.Error)

					http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

					return tx.Error

				}

				now := time.Now()

				user.Name = user_data.Name

				user.Email = user_data.Email

				user.Avatar = user_data.Avatar

				user.EmailConfirmed = true

				user.EmailConfirmedAt = &now

				if err = user.Create(tx); err != nil {

					logrus.Error(err)

					http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

					return err

				}

			}

			payload := &map[string]interface{}{
				"event":    "login",
				"provider": oauth_provider.name(),
				"user":     user,
			}

			hook_response, err := hook.TriggerHook("login", payload, config)

			if err != nil {

				logrus.Error(err)

				http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

				return err

			}

			jwt := jwt.New(state.Provider, user, hook_response, config.JWT)

			signed_token, err := jwt.Sign()

			if err != nil {

				logrus.Error(err)

				http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

				return err

			}

			refresh_token := model.RefreshToken{
				Token:  randstr.String(50),
				UserID: user.ID,
			}

			if err := refresh_token.Create(db); err != nil {

				logrus.Error(err)

				http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

				return err

			}

			log := model.NewLog(user.ID, "login", log_data.IP, nil, log_data.Location, log_data.UserAgent)

			if err := user.SignedIn(tx, log); err != nil {

				logrus.Error(err)

				http.Redirect(res, req, internal_redirect, http.StatusTemporaryRedirect)

				return err

			}

			cookie := &http.Cookie{
				HttpOnly: true,
				Name:     config.RefreshTokenCookieName,
				Value:    refresh_token.Token,
			}

			http.SetCookie(res, cookie)

			redirect_url := fmt.Sprintf("%s/%s?access_token=%s&id=%s", config.SiteURL, config.SocialRedirectPage, signed_token, user.ID)

			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)

			return nil

		})

	}

}

package handlers

import (
	"net/http"
	"time"

	"github.com/sirupsen/logrus"
	"github.com/thanhpk/randstr"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/hook"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/lib/mail"
	"github.com/zolamk/trust/lib/sms"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

func SignIn(user *model.User, ip, ua string, tx *gorm.DB, res http.ResponseWriter) (string, string, error) {

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

	hook_response, err := hook.TriggerHook(hook_user.ID, "login", payload, user.Config)

	if err != nil {

		logrus.Error(err)

		e := ErrWebHook

		e.Message = err.Error()

		return "", "", e

	}

	token := jwt.New("password", user, hook_response, user.Config)

	signed_token, err := token.Sign()

	if err != nil {

		logrus.Error(err)

		return "", "", ErrInternal

	}

	refresh_token := model.RefreshToken{
		Token:  randstr.String(50),
		UserID: user.ID,
	}

	if err = refresh_token.Create(tx); err != nil {

		logrus.Error(err)

		return "", "", ErrInternal

	}

	log := model.NewLog(user.ID, "login", ip, nil, ua)

	if err = user.SignedIn(tx, log); err != nil {

		logrus.Error(err)

		return "", "", ErrInternal

	}

	if user.Config.SetCookies {

		cookie := &http.Cookie{
			HttpOnly: true,
			Secure:   true,
			Path:     "/",
			Name:     user.Config.RefreshTokenCookieName,
			SameSite: http.SameSiteStrictMode,
			Value:    refresh_token.Token,
			Expires:  time.Now().Add(time.Hour * 24 * 7),
			Domain:   user.Config.RefreshTokenCookieDomain,
		}

		http.SetCookie(res, cookie)

		cookie = &http.Cookie{
			HttpOnly: true,
			Secure:   true,
			Path:     "/",
			Name:     user.Config.AccessTokenCookieName,
			SameSite: http.SameSiteStrictMode,
			Value:    signed_token,
			Expires:  token.ExpiresAt.Time,
			Domain:   user.Config.AccessTokenCookieDomain,
		}

		http.SetCookie(res, cookie)

	}

	return signed_token, refresh_token.Token, nil

}

func ValidatePhone(phone string, tx *gorm.DB, config *config.Config) error {

	if !config.PhoneRule.MatchString(phone) {

		return ErrInvalidPhone

	}

	user := &model.User{}

	if err := tx.First(user, "phone = ?", phone).Error; err != nil {

		if err == gorm.ErrRecordNotFound {

			return nil

		}

		return ErrInternal

	}

	err := ErrPhoneRegistered

	err.Extensions["email"] = user.Email

	err.Extensions["phone"] = user.Phone

	err.Extensions["id"] = user.ID

	return err

}

func ValidateEmail(email string, tx *gorm.DB, config *config.Config) error {

	if !config.EmailRule.MatchString(email) {

		return ErrInvalidEmail

	}

	user := &model.User{}

	if err := tx.First(user, "email = ?", email).Error; err != nil {

		if err == gorm.ErrRecordNotFound {

			return nil

		}

		logrus.Error(err)

		return ErrInternal

	}

	err := ErrEmailRegistered

	err.Extensions["email"] = user.Email

	err.Extensions["phone"] = user.Phone

	err.Extensions["id"] = user.ID

	return err

}

func SendEmailConfirmation(user *model.User, tx *gorm.DB, config *config.Config) error {

	if err := user.InitEmailConfirmation(tx); err != nil {

		return ErrInternal

	}

	context := map[string]string{
		"email":                    *user.Email,
		"site_url":                 config.SiteURL,
		"email_confirmation_token": *user.EmailConfirmationToken,
		"instance_url":             config.InstanceURL,
	}

	if user.Name != nil {

		context["name"] = *user.Name

	}

	if err := mail.SendEmail(config.ConfirmationTemplate, context, *user.Email, config.SMTP); err != nil {

		return ErrInternal

	}

	return nil

}

func SendPhoneConfirmation(user *model.User, tx *gorm.DB, config *config.Config) error {

	if err := user.InitPhoneConfirmation(tx); err != nil {

		return ErrInternal

	}

	context := map[string]string{
		"phone":                    *user.Phone,
		"site_url":                 config.SiteURL,
		"phone_confirmation_token": *user.PhoneConfirmationToken,
		"instance_url":             config.InstanceURL,
	}

	if user.Name != nil {

		context["name"] = *user.Name

	}

	if err := sms.SendSMS(config.ConfirmationTemplate, *user.Phone, context, config.SMS); err != nil {

		return ErrInternal

	}

	return nil

}

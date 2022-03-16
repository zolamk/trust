package user

import (
	"net/http"
	"time"

	"github.com/zolamk/trust/config"
	"gorm.io/gorm"
)

func Logout(db *gorm.DB, config *config.Config, writer http.ResponseWriter) (*bool, error) {

	cookie := &http.Cookie{
		HttpOnly: true,
		Secure:   true,
		Name:     config.RefreshTokenCookieName,
		SameSite: http.SameSiteStrictMode,
		Expires:  time.Unix(0, 0),
		Value:    "",
	}

	http.SetCookie(writer, cookie)

	cookie = &http.Cookie{
		HttpOnly: true,
		Secure:   true,
		Name:     config.AccessTokenCookieName,
		SameSite: http.SameSiteStrictMode,
		Value:    "",
		Expires:  time.Unix(0, 0),
		Domain:   config.AccessTokenCookieDomain,
	}

	http.SetCookie(writer, cookie)

	return nil, nil

}

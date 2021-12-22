package middleware

import (
	"context"
	"net/http"

	"github.com/zolamk/trust/config"
)

func AttachRefreshToken(config *config.Config) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {

		return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

			cookies := req.Cookies()

			var refresh_token string

			for _, cookie := range cookies {
				if cookie.Name == config.RefreshTokenCookieName {
					refresh_token = cookie.Value
					break
				}
			}

			ctx := context.WithValue(req.Context(), RefreshTokenKey, refresh_token)

			req = req.WithContext(ctx)

			next.ServeHTTP(res, req)

		})

	}

}

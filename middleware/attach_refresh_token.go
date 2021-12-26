package middleware

import (
	"context"
	"net/http"

	"github.com/zolamk/trust/config"
)

func AttachRefreshToken(config *config.Config) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {

		return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

			cookie, err := req.Cookie(config.RefreshTokenCookieName)

			if err != nil {
				next.ServeHTTP(res, req)
				return
			}

			ctx := context.WithValue(req.Context(), RefreshTokenKey, cookie.Value)

			req = req.WithContext(ctx)

			next.ServeHTTP(res, req)

		})

	}

}

package middleware

import (
	"context"
	"net/http"
	"strings"

	"github.com/sirupsen/logrus"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/jwt"
)

func Authenticated(config *config.Config) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {

		return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

			authorization := req.Header.Get("authorization")

			if authorization == "" {

				cookie, err := req.Cookie(config.AccessTokenCookieName)

				if err != nil {

					next.ServeHTTP(res, req)

					return

				}

				authorization = cookie.Value

			}

			parts := strings.Split(authorization, "Bearer ")

			if len(parts) == 0 {

				next.ServeHTTP(res, req)

				return

			}

			authorization = parts[len(parts)-1]

			token, err := jwt.Decode(authorization, config.JWT)

			if err != nil {

				logrus.Error(err)

				next.ServeHTTP(res, req)

				return

			}

			ctx := context.WithValue(req.Context(), TokenKey, token)

			ctx = context.WithValue(ctx, ProviderKey, token.Provider)

			req = req.WithContext(ctx)

			next.ServeHTTP(res, req)

		})

	}

}

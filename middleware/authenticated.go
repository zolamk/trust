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
				next.ServeHTTP(res, req)
				return
			}

			parts := strings.Split(authorization, " ")

			if len(parts) != 2 {
				next.ServeHTTP(res, req)
				return
			}

			authorization = parts[1]

			token, err := jwt.Decode(authorization, config.JWT)

			if err != nil {
				logrus.Error(err)
				next.ServeHTTP(res, req)
				return
			}

			ctx := context.WithValue(req.Context(), "token", token)

			req = req.WithContext(ctx)

			next.ServeHTTP(res, req)

		})

	}

}

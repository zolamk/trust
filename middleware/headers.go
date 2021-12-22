package middleware

import (
	"context"
	"net/http"
	"strings"
)

func Headers(next http.Handler) http.Handler {

	return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

		ip := req.Header.Get("x-real-ip")

		if ip == "" {

			ip = req.Header.Get("x-forwarded-for")

			if ip == "" {

				ip = strings.Split(req.RemoteAddr, ":")[0]

			}

		}

		ctx := context.WithValue(req.Context(), IPKey, ip)

		ctx = context.WithValue(ctx, UserAgentKey, req.UserAgent())

		req = req.WithContext(ctx)

		next.ServeHTTP(res, req)

	})

}

package middleware

import (
	"context"
	"net/http"
	"strings"
)

type LogData struct {
	IP        string
	UserAgent string
}

func AttachLogData(next http.Handler) http.Handler {

	return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

		ip := req.Header.Get("x-real-ip")

		if ip == "" {

			ip = req.Header.Get("x-forwarded-for")

			if ip == "" {

				ip = strings.Split(req.RemoteAddr, ":")[0]

			}

		}

		// since there maybe multiple ips for multiple proxies
		// get the left most ip as it is the clients ip
		ip = strings.TrimSpace(strings.Split(ip, ",")[0])

		ctx := context.WithValue(req.Context(), LogDataKey, LogData{
			ip,
			req.UserAgent(),
		})

		req = req.WithContext(ctx)

		next.ServeHTTP(res, req)

	})

}

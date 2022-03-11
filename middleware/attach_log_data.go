package middleware

import (
	"context"
	"net/http"
	"strings"

	"github.com/ip2location/ip2location-go/v9"
	ua "github.com/mileusna/useragent"
	"github.com/sirupsen/logrus"
)

type LogData struct {
	IP        string
	Location  *ip2location.IP2Locationrecord
	UserAgent *ua.UserAgent
}

func AttachLogData(ip2location_db *ip2location.DB) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {

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

			location, err := ip2location_db.Get_all(ip)

			if err != nil {
				logrus.Error(err)
			}

			ua := ua.Parse(req.UserAgent())

			ctx := context.WithValue(req.Context(), LogDataKey, LogData{
				ip,
				&location,
				&ua,
			})

			req = req.WithContext(ctx)

			next.ServeHTTP(res, req)

		})

	}

}

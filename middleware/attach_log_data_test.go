package middleware

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zolamk/trust/config"
)

func TestAttachLogData(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	real_ip := "196.188.12.61"

	req.Header.Add("x-real-ip", real_ip)

	config, _ := config.New("../test/configs/complete.conf")

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(real_ip, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(config.IP2LocationDB)(handler).ServeHTTP(res, req)

	req.Header.Del("x-real-ip")

	forwarded_for := "196.188.12.71"

	req.Header.Add("x-forwarded-for", forwarded_for)

	handler = http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(forwarded_for, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(config.IP2LocationDB)(handler).ServeHTTP(res, req)

	req.Header.Del("x-forwarded-for")

	remote_addr := "196.188.12.81"

	req.RemoteAddr = remote_addr + ":1995"

	handler = http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(remote_addr, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(config.IP2LocationDB)(handler).ServeHTTP(res, req)

}

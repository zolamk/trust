package middleware

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAttachLogData(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	real_ip := "196.188.12.61"

	req.Header.Add("x-real-ip", real_ip)

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(real_ip, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(handler).ServeHTTP(res, req)

	req.Header.Del("x-real-ip")

	forwarded_for := "196.188.12.71"

	req.Header.Add("x-forwarded-for", forwarded_for)

	handler = http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(forwarded_for, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(handler).ServeHTTP(res, req)

	req.Header.Del("x-real-ip")

	forwarded_for = "248.116.107.103, 10.42.0.1"

	req.Header.Set("x-forwarded-for", forwarded_for)

	handler = http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal("248.116.107.103", log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(handler).ServeHTTP(res, req)

	req.Header.Del("x-forwarded-for")

	remote_addr := "196.188.12.81"

	req.RemoteAddr = remote_addr + ":1995"

	handler = http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		log_data, ok := r.Context().Value(LogDataKey).(LogData)

		assert.Equal(true, ok, "expected log data to be present in request context")

		assert.Equal(remote_addr, log_data.IP, "expected ip in log data to match x-real-ip header")

	})

	AttachLogData(handler).ServeHTTP(res, req)

}

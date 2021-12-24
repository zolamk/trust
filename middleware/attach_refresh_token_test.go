package middleware

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zolamk/trust/config"
)

func TestAttachRefreshToken(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	refresh_token := "refresh_token"

	config, _ := config.New("../test/configs/complete.conf")

	req.AddCookie(&http.Cookie{
		Name:  config.RefreshTokenCookieName,
		Value: refresh_token,
	})

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		rf, ok := r.Context().Value(RefreshTokenKey).(string)

		assert.Equal(true, ok, "expected refresh token to be present in request context")

		assert.Equal(refresh_token, rf, "expected refresh token in context to match")

	})

	AttachRefreshToken(config)(handler).ServeHTTP(res, req)

}

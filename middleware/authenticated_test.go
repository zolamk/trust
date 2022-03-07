package middleware

import (
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
)

func TestAuthenticated(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	config, _ := config.New("../test/configs/complete.conf")

	token := jwt.New("password", &model.User{}, nil, config)

	signed_token, _ := token.Sign()

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", signed_token))

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		_, ok := r.Context().Value(TokenKey).(*jwt.JWT)

		assert.Equal(true, ok, "expected token to be present in request context")

	})

	Authenticated(config)(handler).ServeHTTP(res, req)

}

func TestAuthenticatedCookie(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	config, _ := config.New("../test/configs/complete.conf")

	token := jwt.New("password", &model.User{}, nil, config)

	signed_token, _ := token.Sign()

	req.AddCookie(&http.Cookie{
		Name:  config.AccessTokenCookieName,
		Value: signed_token,
	})

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		_, ok := r.Context().Value(TokenKey).(*jwt.JWT)

		assert.Equal(true, ok, "expected token to be present in request context")

	})

	Authenticated(config)(handler).ServeHTTP(res, req)

}

func TestAuthenticatedAuthorizationHeaderMissing(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	config, _ := config.New("../test/configs/complete.conf")

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		_, ok := r.Context().Value(TokenKey).(*jwt.JWT)

		assert.Equal(false, ok, "expected token not to be present in request context")

	})

	Authenticated(config)(handler).ServeHTTP(res, req)

}

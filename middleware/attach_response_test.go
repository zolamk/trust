package middleware

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAttachResponse(t *testing.T) {

	assert := assert.New(t)

	req := httptest.NewRequest("POST", "http://example.com", nil)

	res := httptest.NewRecorder()

	handler := http.HandlerFunc(func(rw http.ResponseWriter, r *http.Request) {

		_, ok := r.Context().Value(WriterKey).(http.ResponseWriter)

		assert.Equal(true, ok, "expected response writer to be present in request context")

	})

	AttachResponse(handler).ServeHTTP(res, req)

}

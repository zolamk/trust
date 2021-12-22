package middleware

import (
	"context"
	"net/http"
)

func AttachResponse(next http.Handler) http.Handler {

	return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

		ctx := context.WithValue(req.Context(), WriterKey, res)

		req = req.WithContext(ctx)

		next.ServeHTTP(res, req)

	})

}

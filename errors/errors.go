package errors

import e "errors"

var (
	Internal        = e.New("internal server error")
	TooManyRequests = e.New("too many requests")
)

package errors

import e "errors"

var (
	Internal                    = e.New("internal server error")
	TooManyRequests             = e.New("too many requests")
	IncorrectUsernameOrPassword = e.New("incorrect username or password")
	EmailNotConfirmed           = e.New("email not confirmed")
	PhoneNotConfirmed           = e.New("phone not confirmed")
	WebHook                     = e.New("webhook error")
)

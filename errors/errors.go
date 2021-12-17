package errors

import e "errors"

var (
	Internal                    = e.New("internal server error")
	TooManyRequests             = e.New("too many requests")
	IncorrectUsernameOrPassword = e.New("incorrect username or password")
	EmailNotConfirmed           = e.New("email not confirmed")
	PhoneNotConfirmed           = e.New("phone not confirmed")
	WebHook                     = e.New("webhook error")
	RefreshTokenNotFound        = e.New("refresh token not found")
	InvalidEmail                = e.New("invalid email address")
	InvalidPhone                = e.New("invalid phone number")
	InvalidPassword             = e.New("invalid password")
	EmailDisabled               = e.New("email disabled")
	PhoneDisabled               = e.New("phone disabled")
	EmailRegistered             = e.New("email address registered")
	PhoneRegistered             = e.New("phone number registered")
)

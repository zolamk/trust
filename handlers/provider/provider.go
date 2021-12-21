package provider

import "golang.org/x/oauth2"

type Provider interface {
	enabled() bool
	name() string
	get_config() *oauth2.Config
	get_user_data(access_token string) (*UserData, error)
}

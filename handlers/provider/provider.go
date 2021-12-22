package provider

import (
	"errors"

	"github.com/zolamk/trust/config"
	"golang.org/x/oauth2"
)

type Provider interface {
	enabled() bool
	name() string
	get_config() *oauth2.Config
	get_user_data(string) (*UserData, error)
}

func get_provider(name string, config *config.Config) (Provider, error) {
	switch name {
	case "facebook":
		return &FacebookProvider{config}, nil
	case "google":
		return &GoogleProvider{config}, nil
	case "github":
		return &GithubProvider{config}, nil
	default:
		return nil, errors.New("unknown provider")
	}
}

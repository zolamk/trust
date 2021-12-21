package provider

import (
	"fmt"
	"net/http"

	"github.com/zolamk/trust/config"
	"gorm.io/gorm"
)

func Authorize(db *gorm.DB, config *config.Config) http.Handler {

	return http.HandlerFunc(func(res http.ResponseWriter, req *http.Request) {

		var oauth_provider Provider

		provider_disabled := fmt.Sprintf("%s/%s?error=provider_disabled", config.SiteURL, config.SocialRedirectPage)

		switch req.URL.Query().Get("provider") {
		case "facebook":
			if !config.FacebookEnabled {
				http.Redirect(res, req, provider_disabled, http.StatusTemporaryRedirect)
				return
			}
			oauth_provider = NewFacebookProvider(config)
		case "google":
			if !config.GoogleEnabled {
				http.Redirect(res, req, provider_disabled, http.StatusTemporaryRedirect)
				return
			}
			oauth_provider = NewGoogleProvider(config)
		default:
			redirect_url := fmt.Sprintf("%s/%s?error=unknown_provider", config.SiteURL, config.SocialRedirectPage)
			http.Redirect(res, req, redirect_url, http.StatusTemporaryRedirect)
			return
		}

		provider := &providerState{
			config:   config,
			Provider: oauth_provider.name(),
		}

		state, err := provider.sign()

		if err != nil {
			http.Redirect(res, req, "", http.StatusTemporaryRedirect)
		}

		config := oauth_provider.get_config()

		url := config.AuthCodeURL(*state)

		http.Redirect(res, req, url, http.StatusTemporaryRedirect)

	})

}

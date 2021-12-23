package provider

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/zolamk/trust/config"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
)

type googleUser struct {
	Name          *string `json:"name"`
	Email         *string `json:"email"`
	VerifiedEmail bool    `json:"verified_email"`
	Picture       *string `json:"picture"`
}

type GoogleProvider struct {
	c *config.Config
}

func NewGoogleProvider(c *config.Config) *GoogleProvider {
	return &GoogleProvider{
		c,
	}
}

func (g *GoogleProvider) name() string {
	return "google"
}

func (g *GoogleProvider) enabled() bool {
	return g.c.GoogleEnabled
}

func (g *GoogleProvider) get_config() *oauth2.Config {
	return &oauth2.Config{
		ClientID:     g.c.GoogleClientID,
		ClientSecret: g.c.GoogleClientSecret,
		Endpoint:     google.Endpoint,
		Scopes:       []string{"email", "profile"},
		RedirectURL:  fmt.Sprintf("%s/authorize/callback", g.c.InstanceURL),
	}
}

func (g *GoogleProvider) get_user_data(access_token string) (*UserData, error) {

	req, err := http.NewRequest("GET", "https://www.googleapis.com/oauth2/v1/userinfo?alt=json", nil)

	if err != nil {
		return nil, err
	}

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", access_token))

	res, err := http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	decoder := json.NewDecoder(res.Body)

	google_user := &googleUser{}

	if err := decoder.Decode(google_user); err != nil {
		return nil, err
	}

	return &UserData{
		Name:     google_user.Name,
		Email:    google_user.Email,
		Avatar:   google_user.Picture,
		Verified: google_user.VerifiedEmail,
	}, nil
}

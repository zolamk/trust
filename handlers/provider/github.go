package provider

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/zolamk/trust/config"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/github"
)

type githubUser struct {
	Name      *string `json:"name"`
	Email     *string `json:"email"`
	AvatarURL *string `json:"avatar_url"`
}

type githubUserEmail struct {
	Email    string
	Primary  bool
	Verified bool
}

type GithubProvider struct {
	c *config.Config
}

func (g *GithubProvider) name() string {
	return "github"
}

func (g *GithubProvider) enabled() bool {
	return g.c.Github.Enabled
}

func (g *GithubProvider) get_config() *oauth2.Config {
	return &oauth2.Config{
		ClientID:     g.c.Github.ID,
		ClientSecret: g.c.Github.Secret,
		Endpoint:     github.Endpoint,
		Scopes:       []string{"user:email"},
		RedirectURL:  fmt.Sprintf("%s/authorize/callback", g.c.InstanceURL),
	}
}

func (g *GithubProvider) get_user_data(access_token string) (*UserData, error) {

	req, err := http.NewRequest("GET", "https://api.github.com/user", nil)

	if err != nil {
		return nil, err
	}

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", access_token))

	res, err := http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	decoder := json.NewDecoder(res.Body)

	github_user := &githubUser{}

	if err := decoder.Decode(github_user); err != nil {
		return nil, err
	}

	req, err = http.NewRequest("GET", "https://api.github.com/user/emails", nil)

	if err != nil {
		return nil, err
	}

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", access_token))

	res, err = http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	decoder = json.NewDecoder(res.Body)

	github_user_emails := []*githubUserEmail{}

	if err = decoder.Decode(&github_user_emails); err != nil {
		return nil, err
	}

	data := &UserData{
		Name:     github_user.Name,
		Verified: false,
		Avatar:   github_user.AvatarURL,
	}

	for _, email := range github_user_emails {
		if email.Primary {
			data.Email = &email.Email
			data.Verified = email.Verified
			break
		}
	}

	return data, nil

}

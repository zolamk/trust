package provider

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/zolamk/trust/config"
	"golang.org/x/oauth2"
	"golang.org/x/oauth2/facebook"
)

type PictureData struct {
	IsSilhouette bool   `json:"is_silhouette"`
	URL          string `json:"url"`
}

type Data struct {
	Data PictureData `json:"data"`
}

type FacebookUser struct {
	Email   *string `json:"email"`
	Name    *string `json:"name"`
	Picture Data    `json:"picture"`
}

type FacebookProvider struct {
	config *config.Config
}

func (f *FacebookProvider) name() string {
	return "facebook"
}

func (f *FacebookProvider) enabled() bool {
	return f.config.Facebook.Enabled
}

func (f *FacebookProvider) get_config() *oauth2.Config {
	return &oauth2.Config{
		ClientID:     f.config.Facebook.ID,
		ClientSecret: f.config.Facebook.Secret,
		Scopes:       []string{"email"},
		Endpoint:     facebook.Endpoint,
		RedirectURL:  fmt.Sprintf("%s/authorize/callback", f.config.InstanceURL),
	}
}

func (f *FacebookProvider) get_user_data(access_token string) (*UserData, error) {

	req, err := http.NewRequest("GET", "https://graph.facebook.com/me?fields=name,email,picture{url,is_silhouette}", nil)

	if err != nil {
		return nil, err
	}

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", access_token))

	res, err := http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	defer res.Body.Close()

	decoder := json.NewDecoder(res.Body)

	data := &FacebookUser{}

	if err := decoder.Decode(data); err != nil {
		return nil, err
	}

	user_data := &UserData{
		Name:     data.Name,
		Email:    data.Email,
		Verified: true,
	}

	if !data.Picture.Data.IsSilhouette {
		user_data.Avatar = &data.Picture.Data.URL
	}

	return user_data, nil

}

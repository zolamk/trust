package hook

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/zolamk/trust/config"
)

func TriggerHook(event string, payload *map[string]interface{}, config *config.Config) (*interface{}, error) {

	url := config.LoginHook

	now := time.Now()

	if url == nil {
		return nil, nil
	}

	alg := jwt.GetSigningMethod(config.JWT.Alg)

	claims := struct {
		jwt.RegisteredClaims
		Metadata map[string][]string `json:"metdata"`
	}{
		jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(now.Add(time.Minute)),
			Audience:  jwt.ClaimStrings{config.JWT.Aud},
			IssuedAt:  jwt.NewNumericDate(now),
		},
		map[string][]string{
			"roles": {"trust"},
		},
	}

	token := jwt.NewWithClaims(alg, claims)

	token_string, err := token.SignedString(config.JWT.GetSigningKey())

	if err != nil {
		return nil, err
	}

	body, err := json.Marshal(payload)

	if err != nil {
		return nil, err
	}

	req, err := http.NewRequest(http.MethodPost, *url, bytes.NewReader(body))

	if err != nil {
		return nil, err
	}

	req.Header.Add("content-type", "application/json")

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", token_string))

	res, err := http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	if res.StatusCode >= 400 {
		return nil, errors.New("webhook error")
	}

	if res.Header.Get("content-type") != "application/json" {
		return nil, nil
	}

	decoder := json.NewDecoder(res.Body)

	var hook_response interface{}

	if err := decoder.Decode(&hook_response); err != nil {
		if err == io.EOF {
			return nil, nil
		}
		return nil, err
	}

	return &hook_response, nil

}

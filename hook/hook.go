package hook

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/ohler55/ojg/oj"
	"github.com/zolamk/trust/config"
)

func TriggerHook(user_id string, event string, payload *map[string]interface{}, config *config.Config) (*interface{}, error) {

	url := config.LoginHook

	now := time.Now()

	if url == "" {
		return nil, nil
	}

	alg := jwt.GetSigningMethod(config.JWT.Alg)

	claims := struct {
		jwt.RegisteredClaims
		Metadata map[string][]string `json:"metadata"`
	}{
		jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(now.Add(time.Minute)),
			Audience:  jwt.ClaimStrings{config.JWT.Aud},
			IssuedAt:  jwt.NewNumericDate(now),
			Subject:   user_id,
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

	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))

	if err != nil {
		return nil, err
	}

	req.Header.Add("content-type", "application/json")

	req.Header.Add("authorization", fmt.Sprintf("Bearer %s", token_string))

	req.AddCookie(&http.Cookie{
		Name:   config.AccessTokenCookieName,
		Domain: config.AccessTokenCookieDomain,
		Value:  token_string,
	})

	res, err := http.DefaultClient.Do(req)

	if err != nil {
		return nil, err
	}

	if res.StatusCode >= 400 {
		return nil, errors.New("webhook error")
	}

	if !strings.Contains(res.Header.Get("content-type"), "application/json") {
		return nil, nil
	}

	var decoder oj.Parser

	var hook_response interface{}

	if hook_response, err = decoder.ParseReader(res.Body); err != nil {
		if err == io.EOF {
			return nil, nil
		}
		return nil, err
	}

	if config.MetadataPath != nil {

		result := config.MetadataPath.Get(hook_response)

		if len(result) > 0 {
			return &result[0], nil
		}

		return nil, nil

	}

	return &hook_response, nil

}

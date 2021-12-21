package provider

import (
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/zolamk/trust/config"
)

type providerState struct {
	jwt.RegisteredClaims
	config   *config.Config
	Provider string `json:"provider"`
}

func (p *providerState) sign() (*string, error) {

	p.RegisteredClaims.ExpiresAt = jwt.NewNumericDate(time.Now().Add(time.Minute * 5))

	p.Audience = jwt.ClaimStrings{p.config.JWT.Aud}

	alg := jwt.GetSigningMethod(p.config.JWT.Alg)

	token := jwt.NewWithClaims(alg, p)

	signed_string, err := token.SignedString(p.config.JWT.GetSigningKey())

	return &signed_string, err

}

func verify(state string, config *config.Config) (*providerState, error) {

	claims := &providerState{}

	token, err := jwt.ParseWithClaims(state, claims, func(t *jwt.Token) (interface{}, error) {
		return config.JWT.GetDecodingKey(), nil
	})

	if err != nil {
		return nil, err
	}

	if !token.Valid {
		return nil, err
	}

	claims, ok := token.Claims.(*providerState)

	if !ok {
		return nil, err
	}

	return claims, nil

}

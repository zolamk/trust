package jwt

import (
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/model"
)

type JWT struct {
	jwt.RegisteredClaims
	Name     *string      `json:"name,omitempty"`
	Email    *string      `json:"email,omitempty"`
	Phone    *string      `json:"phone,omitempty"`
	Metadata *interface{} `json:"metadata,omitempty"`
	config   *config.JWTConfig
}

func New(user *model.User, metadata *interface{}, config *config.JWTConfig) *JWT {

	now := time.Now()

	return &JWT{
		jwt.RegisteredClaims{
			Audience:  jwt.ClaimStrings{config.Aud},
			ExpiresAt: jwt.NewNumericDate(now.Add(time.Second * config.Exp)),
			Issuer:    config.Iss,
			Subject:   user.ID,
		},
		user.Name,
		user.Email,
		user.Phone,
		metadata,
		config,
	}

}

func Decode(signed_string string, config *config.JWTConfig) (*JWT, error) {

	claims := &JWT{}

	token, err := jwt.ParseWithClaims(signed_string, claims, func(t *jwt.Token) (interface{}, error) {
		return config.GetDecodingKey(), nil
	})

	if err != nil {
		return nil, err
	}

	if !token.Valid {
		return nil, errors.ErrInvalidJWT
	}

	claims, ok := token.Claims.(*JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return claims, nil

}

func (j *JWT) Sign() (string, error) {

	alg := jwt.GetSigningMethod(j.config.Alg)

	token := jwt.NewWithClaims(alg, j)

	return token.SignedString(j.config.GetSigningKey())

}

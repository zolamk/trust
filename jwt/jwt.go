package jwt

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/model"
)

type JWT struct {
	jwt.RegisteredClaims
	Name     *string      `json:"name,omitempty"`
	Email    *string      `json:"email,omitempty"`
	Phone    *string      `json:"phone,omitempty"`
	Metadata *interface{} `json:"metadata,omitempty"`
	Provider string       `json:"provider"`
	config   *config.Config
}

func New(provider string, user *model.User, metadata *interface{}, config *config.Config) *JWT {

	now := time.Now()

	return &JWT{
		jwt.RegisteredClaims{
			Audience:  jwt.ClaimStrings{config.JWT.Aud},
			ExpiresAt: jwt.NewNumericDate(now.Add(time.Minute * config.JWT.Exp)),
			Issuer:    config.JWT.Iss,
			Subject:   user.ID,
		},
		user.Name,
		user.Email,
		user.Phone,
		metadata,
		provider,
		config,
	}

}

func Decode(signed_string string, config *config.Config) (*JWT, error) {

	claims := &JWT{
		config: config,
	}

	token, err := jwt.ParseWithClaims(signed_string, claims, func(t *jwt.Token) (interface{}, error) {
		return config.JWT.GetDecodingKey(), nil
	})

	if err != nil {
		return nil, err
	}

	if !token.Valid {
		return nil, errors.New("invalid jwt")
	}

	claims, ok := token.Claims.(*JWT)

	if !ok {
		return nil, errors.New("couldn't case jwt claims")
	}

	return claims, nil

}

func (j *JWT) Sign() (string, error) {

	alg := jwt.GetSigningMethod(j.config.JWT.Alg)

	token := jwt.NewWithClaims(alg, j)

	return token.SignedString(j.config.JWT.GetSigningKey())

}

func (j *JWT) roles() []interface{} {

	roles := []interface{}{}

	if j.Metadata == nil {
		return roles
	}

	results := j.config.RolesPath.Get(*j.Metadata)

	switch rs := results[0].(type) {
	case []interface{}:
		roles = rs
	default:
		return roles
	}

	return roles

}

func (j *JWT) HasReadRole() bool {

	roles := j.roles()

	for _, v := range roles {

		for _, r := range j.config.ReadOnlyRoles {

			if v == r {
				return true
			}

		}

	}

	return false

}

func (j *JWT) HasAdminRole() bool {

	roles := j.roles()

	for _, v := range roles {

		for _, r := range j.config.AdminRoles {

			if v == r {
				return true
			}

		}

	}

	return false

}

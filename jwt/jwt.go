package jwt

import (
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

func (j *JWT) Sign() (string, error) {

	alg := jwt.GetSigningMethod(j.config.Alg)

	token := jwt.NewWithClaims(alg, j)

	return token.SignedString(j.config.GetSigningKey())

}

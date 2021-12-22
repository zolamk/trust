package jwt

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/model"
	"gorm.io/gorm"
)

type JWT struct {
	jwt.RegisteredClaims
	Name     *string      `json:"name,omitempty"`
	Email    *string      `json:"email,omitempty"`
	Phone    *string      `json:"phone,omitempty"`
	Metadata *interface{} `json:"metadata,omitempty"`
	Provider string       `json:"provider"`
	config   *config.JWTConfig
}

func New(provider string, user *model.User, metadata *interface{}, config *config.JWTConfig) *JWT {

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
		provider,
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
		return nil, errors.New("invalid jwt")
	}

	claims, ok := token.Claims.(*JWT)

	if !ok {
		return nil, errors.New("couldn't case jwt claims")
	}

	return claims, nil

}

func (j *JWT) Sign() (string, error) {

	alg := jwt.GetSigningMethod(j.config.Alg)

	token := jwt.NewWithClaims(alg, j)

	return token.SignedString(j.config.GetSigningKey())

}

func (j *JWT) IsAdmin(db *gorm.DB) (bool, error) {

	is_admin := false

	if tx := db.Table("trust.users").Select("is_admin").Where("id = ?", j.Subject).Scan(&is_admin); tx.Error != nil && tx.Error != gorm.ErrRecordNotFound {
		return false, tx.Error
	}

	return is_admin, nil
}

package jwt

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zolamk/trust/config"
	"github.com/zolamk/trust/model"
)

func TestJWT(t *testing.T) {

	assert := assert.New(t)

	config, _ := config.New("../test/configs/complete.conf")

	user := &model.User{
		ID: "TRUST",
	}

	token := New("password", user, nil, config)

	signed_token, err := token.Sign()

	assert.Equal(nil, err, "Expected err to be nil")

	assert.NotEqual("", signed_token, "Expected signed token not to be empty string")

	_, err = Decode(signed_token, config)

	assert.Equal(nil, err, "Expected err to be nil")

}

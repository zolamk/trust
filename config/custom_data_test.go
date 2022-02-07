package config

import (
	"regexp"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFieldRequired(t *testing.T) {

	assert := assert.New(t)

	var err error

	f := Field{
		Type:     String,
		Required: true,
	}

	err = f.Validate("name", nil)

	assert.Equal("field name is required", err.Error(), `expected validate to return required error but got "%v"`, err)

	f = Field{
		Type:     Integer,
		Required: true,
	}

	err = f.Validate("age", nil)

	assert.Equal("field age is required", err.Error(), `expected validate to return required error but got "%v"`, err)

	f = Field{
		Type:     Float,
		Required: true,
	}

	err = f.Validate("pi", nil)

	assert.Equal("field pi is required", err.Error(), `expected validate to return required error but got "%v"`, err)

	f = Field{
		Type:     Boolean,
		Required: true,
	}

	err = f.Validate("has_agreed", nil)

	assert.Equal("field has_agreed is required", err.Error(), `expected validate to return required error but got "%v"`, err)

}

func TestFieldType(t *testing.T) {

	assert := assert.New(t)

	var err error

	f := Field{
		Type: String,
	}

	err = f.Validate("name", 10)

	assert.Equal("field name expected string but got int", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("name", 3.14)

	assert.Equal("field name expected string but got float64", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("name", true)

	assert.Equal("field name expected string but got bool", err.Error(), `expected validate to return type error but got "%v"`, err)

	f = Field{
		Type: Integer,
	}

	err = f.Validate("age", "10")

	assert.Equal("field age expected integer but got string", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("age", 3.14)

	assert.Equal("field age expected integer but got float64", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("age", true)

	assert.Equal("field age expected integer but got bool", err.Error(), `expected validate to return type error but got "%v"`, err)

	f = Field{
		Type: Float,
	}

	err = f.Validate("pi", "3.14")

	assert.Equal("field pi expected float but got string", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("pi", 3)

	assert.Equal("field pi expected float but got int", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("pi", true)

	assert.Equal("field pi expected float but got bool", err.Error(), `expected validate to return type error but got "%v"`, err)

	f = Field{
		Type: Boolean,
	}

	err = f.Validate("has_agreed", "true")

	assert.Equal("field has_agreed expected boolean but got string", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("has_agreed", 3)

	assert.Equal("field has_agreed expected boolean but got int", err.Error(), `expected validate to return type error but got "%v"`, err)

	err = f.Validate("has_agreed", 3.14)

	assert.Equal("field has_agreed expected boolean but got float64", err.Error(), `expected validate to return type error but got "%v"`, err)

}

func TestFieldMinMax(t *testing.T) {

	assert := assert.New(t)

	var err error

	min := 5.0

	max := 10.0

	f := Field{
		Type: String,
		Min:  &min,
		Max:  &max,
	}

	err = f.Validate("name", "zola")

	assert.Equal("field name length is less than minimum length specified", err.Error(), "expected field length error but got %v", err)

	err = f.Validate("name", "zelalem mekonen")

	assert.Equal("field name length is greater than maximum length specified", err.Error(), "expected field length error but got %v", err)

	err = f.Validate("name", "zelalem")

	assert.Equal(nil, err, "expected field length error to be nil but got %v", err)

	f = Field{
		Type: Integer,
		Min:  &min,
		Max:  &max,
	}

	err = f.Validate("age", int64(4))

	assert.Equal("field age is less than minimum specified", err.Error(), "expected field minimum error but got %v", err)

	err = f.Validate("age", int64(11))

	assert.Equal("field age is greater than maximum specified", err.Error(), "expected field maximum error but got %v", err)

	err = f.Validate("name", int64(8))

	assert.Equal(nil, err, "expected field maximum/minimum error to be nil but got %v", err)

	f = Field{
		Type: Float,
		Min:  &min,
		Max:  &max,
	}

	err = f.Validate("pi", 3.14)

	assert.Equal("field pi is less than minimum specified", err.Error(), "expected field minimum error but got %v", err)

	err = f.Validate("pi", 11.0)

	assert.Equal("field pi is greater than maximum specified", err.Error(), "expected field maximum error but got %v", err)

	err = f.Validate("pi", 8.0)

	assert.Equal(nil, err, "expected field maximum/minimum error to be nil but got %v", err)

}

func TestFieldFormat(t *testing.T) {

	assert := assert.New(t)

	var err error

	f := Field{
		Type: String,
		Format: &Regexp{
			*regexp.MustCompile("ID\\d{5,10}"),
		},
	}

	err = f.Validate("id", "ID001")

	assert.Equal("field id doesn't match format specified", err.Error(), "expected field format error but got %v", err)

	err = f.Validate("id", "ID00001")

	assert.Equal(nil, err, "expected field format error to be nil but got %v", err)

}

func TestFieldChoices(t *testing.T) {

	assert := assert.New(t)

	var err error

	choices := Choices([]string{"male", "female"})

	f := Field{
		Type:    String,
		Choices: &choices,
	}

	err = f.Validate("gender", "camel")

	assert.Equal("field gender value is not specified in choices", err.Error(), "expected field choices error but got %v", err)

	err = f.Validate("gender", "male")

	assert.Equal(nil, err, "expected field choices error to be nil but got %v", err)

	err = f.Validate("gender", "female")

	assert.Equal(nil, err, "expected field choices error to be nil but got %v", err)

}

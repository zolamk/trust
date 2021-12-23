package config

import (
	"testing"
)

var rsaPrivateKeys []string = []string{
	"../test/keys/rs256-private.pem",
	"../test/keys/rs384-private.pem",
	"../test/keys/rs512-private.pem",
}

var esPrivateKeys []string = []string{
	"../test/keys/es256-private.pem",
	"../test/keys/es384-private.pem",
	"../test/keys/es512-private.pem",
}

var publicKeys []string = []string{
	"../test/keys/rs256-public.pem",
	"../test/keys/rs384-public.pem",
	"../test/keys/rs512-public.pem",
	"../test/keys/es256-public.pem",
	"../test/keys/es384-public.pem",
	"../test/keys/es512-public.pem",
}

func TestParsePKCS8PrivateKey(t *testing.T) {

	for _, key := range rsaPrivateKeys {

		_, err := parsePKCS8PrivateKey(key)

		if err != nil {
			t.Error("expected err to be nil, got", err)
		}

	}

	for _, key := range esPrivateKeys {

		_, err := parsePKCS8PrivateKey(key)

		if err == nil {
			t.Error("expected err not to be nil, got nil instead")
		}

	}

}

func TestParsePKIXPublicKey(t *testing.T) {

	for _, key := range publicKeys {

		_, err := parsePKIXPublicKey(key)

		if err != nil {
			t.Error("expected err to be nil, got", err)
		}

	}

}

func TestParseECPrivateKey(t *testing.T) {

	for _, key := range esPrivateKeys {

		_, err := parseECPrivateKey(key)

		if err != nil {
			t.Error("expected err to be nil, got", err)
		}

	}

	for _, key := range rsaPrivateKeys {

		_, err := parseECPrivateKey(key)

		if err == nil {
			t.Error("expected err not to be nil, got nil instead")
		}

	}

}

package config

import (
	"crypto/ecdsa"
	"crypto/x509"
	"encoding/pem"
	"os"
)

func parsePKCS8PrivateKey(path string) (interface{}, error) {

	var err error

	var raw_private_key []byte

	var private_key interface{}

	if raw_private_key, err = os.ReadFile(path); err != nil {
		return nil, err
	}

	block, _ := pem.Decode(raw_private_key)

	if private_key, err = x509.ParsePKCS8PrivateKey(block.Bytes); err != nil {
		if private_key, err = x509.ParsePKCS1PrivateKey(block.Bytes); err != nil {
			return nil, err
		}
	}

	return private_key, nil

}

func parsePKIXPublicKey(path string) (interface{}, error) {

	var err error

	var raw_public_key []byte

	var public_key interface{}

	if raw_public_key, err = os.ReadFile(path); err != nil {
		return nil, err
	}

	block, _ := pem.Decode(raw_public_key)

	if public_key, err = x509.ParsePKIXPublicKey(block.Bytes); err != nil {
		return nil, err
	}

	return public_key, nil

}

func parseECPrivateKey(path string) (*ecdsa.PrivateKey, error) {

	var err error

	var raw_private_key []byte

	var private_key *ecdsa.PrivateKey

	if raw_private_key, err = os.ReadFile(path); err != nil {
		return nil, err
	}

	block, _ := pem.Decode(raw_private_key)

	if private_key, err = x509.ParseECPrivateKey(block.Bytes); err != nil {
		return nil, err
	}

	return private_key, nil

}

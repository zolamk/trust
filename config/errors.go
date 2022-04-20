package config

import "errors"

var ErrAsymmetricKeyPathsNotSet = errors.New("expected jwt_private_key_path and jwt_public_key_path to be set for all supported asymmetric algorithms")
var ErrSymmetricSecretNotSet = errors.New("expected jwt_secret to be set for all symmetric algorithms")
var ErrUnsupportedAlgorithm = errors.New("unsupported algorithm")
var ErrUnableToReadTemplate = errors.New("unable to read template")
var ErrGoogleConfig = errors.New("expected google_client_id, google_client_secret to be set if google provider is enabled")
var ErrFacebookConfig = errors.New("expected facebook_client_id, facebook_client_secret to be set if facebook provider is enabled")
var ErrGithubConfig = errors.New("expected github_client_id, github_client_secret to be set if github providder is enabled")
var ErrPhoneEmailDisabled = errors.New("can't disable email and phone at the same time")
var ErrSMSNotConfigured = errors.New("expected sms to be set if phone support is enabled")
var ErrEmailNotConfigured = errors.New("expected smtp to be set if email support is enabled")
var ErrParsingPrivateKey = errors.New("unable to parse private key")
var ErrParsingPublicKey = errors.New("unable to parse public key")
var ErrInvalidCustomDataSchema = errors.New("invalid custom data schema")

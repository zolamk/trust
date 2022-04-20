package config

import (
	"testing"
)

func TestConfig(t *testing.T) {

	var configs map[string]error = map[string]error{
		"../test/configs/email_phone_disabled.conf":          ErrPhoneEmailDisabled,
		"../test/configs/asymmetric_path_not_set.conf":       ErrAsymmetricKeyPathsNotSet,
		"../test/configs/invalid_private_key_path.conf":      ErrParsingPrivateKey,
		"../test/configs/invalid_public_key_path.conf":       ErrParsingPublicKey,
		"../test/configs/invalid_private_key_path_rsa.conf":  ErrParsingPrivateKey,
		"../test/configs/invalid_public_key_path_rsa.conf":   ErrParsingPublicKey,
		"../test/configs/symmetric_secret_not_set.conf":      ErrSymmetricSecretNotSet,
		"../test/configs/unsupported_algorithm.conf":         ErrUnsupportedAlgorithm,
		"../test/configs/missing_change_template.conf":       ErrUnableToReadTemplate,
		"../test/configs/missing_confirmation_template.conf": ErrUnableToReadTemplate,
		"../test/configs/missing_invitation_template.conf":   ErrUnableToReadTemplate,
		"../test/configs/missing_recovery_template.conf":     ErrUnableToReadTemplate,
		"../test/configs/google_enabled.conf":                ErrGoogleConfig,
		"../test/configs/github_enabled.conf":                ErrGithubConfig,
		"../test/configs/facebook_enabled.conf":              ErrFacebookConfig,
		"../test/configs/sms_missing.conf":                   ErrSMSNotConfigured,
		"../test/configs/smtp_missing.conf":                  ErrEmailNotConfigured,
		"../test/configs/complete.conf":                      nil,
	}

	for config, err := range configs {

		_, e := New(config)

		if err != e {

			t.Errorf("expected %s got, %s", err, e)

		}

	}

}

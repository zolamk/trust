package config

import (
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"os"
	"regexp"
	"time"

	"github.com/sirupsen/logrus"
)

type Regexp struct {
	regexp.Regexp
}

func (r *Regexp) UnmarshalText(text []byte) error {
	regexp, err := regexp.Compile(string(text))
	if err != nil {
		return err
	}
	r.Regexp = *regexp
	return nil
}

func (r *Regexp) MarshalText() ([]byte, error) {
	return []byte(r.Regexp.String()), nil
}

type TemplateConfig struct {
	Path    *string
	Subject string
	Email   *Template
	SMS     *Template
}

type SMTPConfig struct {
	Email    string
	Host     string
	Password string
	Port     uint8
	Username string
}

type SMSMapping struct {
	Source      string
	Destination string
	Message     string
}

type SMSConfig struct {
	URL     string
	Method  string
	Source  string
	Mapping *SMSMapping
	Headers map[string]string
	Extra   map[string]string
}

type JWTConfig struct {
	Aud            string        `json:"audience"`
	Alg            string        `json:"algorithm"`
	Exp            time.Duration `json:"expiry"`
	Iss            string        `json:"issuer"`
	PrivateKeyPath *string       `json:"private_key_path"`
	PublicKeyPath  *string       `json:"public_key_path"`
	Secret         *string       `json:"secret"`
	Type           string        `json:"-"`
	privateKey     interface{}
	publicKey      interface{}
}

type Config struct {
	AutoConfirm           bool            `json:"auto_confirm"`
	DatabaseURL           string          `json:"database_url"`
	DisableSignup         bool            `json:"disable_signup"`
	FacebookEnabled       bool            `json:"facebook_enabled"`
	GoogleEnabled         bool            `json:"google_enabled"`
	GithubEnabled         bool            `json:"github_enabled"`
	FacebookClientID      *string         `json:"facebook_client_id"`
	FacebookClientSecret  *string         `json:"facebook_client_secret"`
	GoogleClientID        *string         `json:"google_client_id"`
	GoogleClientSecret    *string         `json:"google_client_secret"`
	GithubClientID        *string         `json:"github_client_id"`
	GithubClientSecret    *string         `json:"github_client_secret"`
	Host                  string          `json:"host"`
	InstanceURL           string          `json:"instance_url"`
	JWT                   *JWTConfig      `json:"jwt"`
	LogLevel              logrus.Level    `json:"log_level"`
	ConfirmationTemplate  *TemplateConfig `json:"confirmation_template"`
	RecoveryTemplate      *TemplateConfig `json:"recovery_template"`
	ChangeTemplate        *TemplateConfig `json:"change_template"`
	InvitationTemplate    *TemplateConfig `json:"invitation_template"`
	Port                  uint16          `json:"port"`
	PasswordRule          Regexp          `json:"password_rule"`
	EmailRule             Regexp          `json:"email_rule"`
	PhoneRule             Regexp          `json:"phone_rule"`
	SiteURL               string          `json:"site_url"`
	SMTP                  *SMTPConfig     `json:"smtp"`
	DisablePhone          bool            `json:"disable_phone"`
	DisableEmail          bool            `json:"disable_email"`
	PasswordHashCost      uint8           `json:"password_hash_cost"`
	MaxConnectionPoolSize uint8           `json:"max_connection_pool_size"`
	AdminOnlyList         bool            `json:"admin_only_list"`
	MinutesBetweenResend  uint8           `json:"minutes_between_resend"`
	LoginHook             *string         `json:"login_hook"`
	SocialRedirectPage    string          `json:"social_redirect_page"`
	SMS                   *SMSConfig      `json:"sms"`
}

func New() *Config {

	default_confirmation, _ := parseStringTemplate("<h2>Confirm your email</h2><p>Follow this link to confirm your email</p><p><a href='{{ site_url }}?confirmation_token={{ email_confirmation_token }}'>Confirm</a></p>")

	default_invitation, _ := parseStringTemplate("<h2>You have been invited</h2><p>Follow this link to accept your invitation</p><p><a href='{{ site_url }}?invitation_token={{ email_invitation_token }}'>Accept Invite</a></p>")

	default_recovery, _ := parseStringTemplate("<h2>Recover Your Account</h2><p>Follow this link to recover you account</p><p><a href='{{ site_url }}?recovery_token={{ email_recovery_token }}'>Recover</a></p>")

	default_change, _ := parseStringTemplate("<h2>Change Your Email Address<h2><p>Follow this link to confirm your email address change</p><p><a href='{{ site_url }}?change_email_token={{ change_email_token }}'>Confirm</a></p>")

	default_confirmation_sms, _ := parseStringTemplate("Phone confirmation code - {{ phone_confirmation_token }}")

	default_invitation_sms, _ := parseStringTemplate("Invitation acceptance code - {{ phone_invitation_token }}")

	default_recovery_sms, _ := parseStringTemplate("Phone recovery code - {{ phone_recovery_token }}")

	default_change_sms, _ := parseStringTemplate("Phone change code -  {{ phone_change_token }}")

	config := Config{
		AutoConfirm:     false,
		DisableSignup:   false,
		FacebookEnabled: false,
		GoogleEnabled:   false,
		GithubEnabled:   false,
		Host:            "localhost",
		JWT: &JWTConfig{
			Exp:  900,
			Type: "assymetric",
			Aud:  "trust",
			Iss:  "trust",
		},
		LogLevel:              logrus.ErrorLevel,
		Port:                  1995,
		DisablePhone:          false,
		DisableEmail:          false,
		AdminOnlyList:         true,
		SocialRedirectPage:    "social",
		PasswordHashCost:      10,
		MaxConnectionPoolSize: 10,
		MinutesBetweenResend:  1,
		PasswordRule:          Regexp{*regexp.MustCompile(".{8,1000}")},
		EmailRule:             Regexp{*regexp.MustCompile(`^[\w\-\.]+@([\w\-]+\.)+[\w\-]{1,}$`)},
		PhoneRule:             Regexp{*regexp.MustCompile(`\+\d{5,15}`)},
		ConfirmationTemplate: &TemplateConfig{
			Subject: "Confirm Your Account",
			SMS:     default_confirmation_sms,
			Email:   default_confirmation,
		},
		InvitationTemplate: &TemplateConfig{
			SMS:     default_invitation_sms,
			Subject: "You've Been Invited",
			Email:   default_invitation,
		},
		RecoveryTemplate: &TemplateConfig{
			SMS:     default_recovery_sms,
			Subject: "Recovery Your Account",
			Email:   default_recovery,
		},
		ChangeTemplate: &TemplateConfig{
			SMS:     default_change_sms,
			Subject: "Confirm Email Change",
			Email:   default_change,
		},
	}

	file, err := os.ReadFile("./.conf")

	if err != nil {
		logrus.Fatalln(err)
	}

	if err = json.Unmarshal(file, &config); err != nil {
		logrus.Fatalln(err)
	}

	switch config.JWT.Alg {
	case "RS256", "RS384", "RS512", "ES256", "ES384", "ES512", "PS256", "PS384", "PS512":

		var public_key []byte

		var private_key []byte

		if config.JWT.PrivateKeyPath == nil || config.JWT.PublicKeyPath == nil {
			logrus.Fatalln("expected jwt_private_key_path and jwt_public_key_path to be set for all supported assymetric algorithms")
		}

		if private_key, err = os.ReadFile(*config.JWT.PrivateKeyPath); err != nil {
			logrus.Fatalln("unable to read private key file " + err.Error())
		}

		if public_key, err = os.ReadFile(*config.JWT.PublicKeyPath); err != nil {
			logrus.Fatalln("unable to read public key file " + err.Error())
		}

		private_block, _ := pem.Decode(private_key)

		public_block, _ := pem.Decode(public_key)

		switch config.JWT.Alg {
		case "RS256", "RS384", "RS512":

			private_key, err := x509.ParsePKCS8PrivateKey(private_block.Bytes)

			if err != nil {
				logrus.Fatalln(err)
			}

			config.JWT.privateKey = private_key

			public_key, err := x509.ParsePKIXPublicKey(public_block.Bytes)

			if err != nil {
				logrus.Fatalln(err)
			}

			config.JWT.publicKey = public_key

		case "ES256", "ES384", "ES512":

			private_key, err := x509.ParseECPrivateKey(private_block.Bytes)

			if err != nil {
				logrus.Fatalln(err)
			}

			config.JWT.privateKey = private_key

			public_key, err := x509.ParsePKIXPublicKey(public_block.Bytes)

			if err != nil {
				logrus.Fatalln(err)
			}

			config.JWT.publicKey = public_key

		}

	case "HS256", "HS384", "HS512":

		if config.JWT.Secret == nil {
			logrus.Fatalln("expected jwt_secret to be set for all symmetric algorithms")
		}

		config.JWT.Type = "symmetric"

	default:
		logrus.Fatalln("unsupported algorithm " + config.JWT.Alg)
	}

	if config.ConfirmationTemplate.Path != nil {

		if config.ConfirmationTemplate.Email, err = parseFileTemplate(*config.ConfirmationTemplate.Path); err != nil {

			logrus.Fatalln("unable to read confirmation email template " + err.Error())

		}

	}

	if config.ChangeTemplate.Path != nil {

		if config.ChangeTemplate.Email, err = parseFileTemplate(*config.ChangeTemplate.Path); err != nil {

			logrus.Fatalln("unable to read change email template " + err.Error())

		}

	}

	if config.RecoveryTemplate.Path != nil {

		if config.RecoveryTemplate.Email, err = parseFileTemplate(*config.RecoveryTemplate.Path); err != nil {

			logrus.Fatalln("unable to read recovery email template", err.Error())

		}

	}

	if config.InvitationTemplate.Path != nil {

		if config.InvitationTemplate.Email, err = parseFileTemplate(*config.InvitationTemplate.Path); err != nil {

			logrus.Fatalln("unable to read invitation email ", err.Error())

		}

	}

	if config.GoogleEnabled && (config.GoogleClientID == nil || config.GoogleClientSecret == nil) {

		logrus.Fatalln("expected google_client_id, google_client_secret to be set if google provider is enabled")

	}

	if config.FacebookEnabled && (config.FacebookClientID == nil || config.FacebookClientSecret == nil) {

		logrus.Fatalln("expected facebook_client_id, facebook_client_secret to be set if facebook provider is enabled")

	}

	if config.GithubEnabled && (config.GithubClientID == nil || config.GithubClientSecret == nil) {

		logrus.Fatalln("expected github_client_id, github_client_secret to be set if github providder is enabled")

	}

	if config.DisableEmail && config.DisablePhone {

		logrus.Fatalln("can't disable email and phone at the same time")

	}

	if !config.DisablePhone && config.SMS == nil {

		logrus.Fatalln("expected sms to be set if phone support is enabled")

	}

	if !config.DisableEmail && config.SMTP == nil {

		logrus.Fatalln("expected smtp to be set if email support is enabled")

	}

	return &config

}

func (c *JWTConfig) GetSigningKey() interface{} {

	if c.Type == "assymetric" {
		return c.privateKey
	}

	return c.Secret

}

func (c *JWTConfig) GetDecodingKey() interface{} {

	if c.Type == "symmetric" {
		return c.publicKey
	}

	return c.Secret

}

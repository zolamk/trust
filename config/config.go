package config

import (
	"encoding/json"
	"log"
	"os"
	"regexp"

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
	Alg            string  `json:"algorithm"`
	Exp            uint64  `json:"exp"`
	PrivateKeyPath *string `json:"private_key_path"`
	PublicKeyPath  *string `json:"public_key_path"`
	Secret         *string `json:"secret"`
	Type           string  `json:"-"`
}

type Config struct {
	Aud                   string          `json:"aud"`
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
	privateKey            *string
	publicKey             *string
	SMS                   *SMSConfig `json:"sms"`
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
		log.Fatalln(err)
	}

	if err = json.Unmarshal(file, &config); err != nil {
		log.Fatalln(err)
	}

	switch config.JWT.Alg {
	case "RS256", "RS384", "RS512", "ES256", "ES384", "ES512":

		if config.JWT.PrivateKeyPath == nil || config.JWT.PublicKeyPath == nil {
			log.Fatalln("expected jwt_private_key_path and jwt_public_key_path to be set for all supported assymetric algorithms")
		}

		if file, err = os.ReadFile(*config.JWT.PrivateKeyPath); err != nil {
			log.Fatalln("unable to read private key file " + err.Error())
		}

		privateKey := string(file)

		config.privateKey = &privateKey

		if file, err = os.ReadFile(*config.JWT.PublicKeyPath); err != nil {
			log.Fatalln("unable to read public key file " + err.Error())
		}

		publicKey := string(file)

		config.publicKey = &publicKey

	case "HS256", "HS384", "HS512":

		if config.JWT.Secret == nil {
			log.Fatalln("expected jwt_secret to be set for all symmetric algorithms")
		}

		config.JWT.Type = "symmetric"

	default:
		log.Fatalln("unsupported algorithm " + config.JWT.Alg)
	}

	if config.ConfirmationTemplate.Path != nil {

		if config.ConfirmationTemplate.Email, err = parseFileTemplate(*config.ConfirmationTemplate.Path); err != nil {

			log.Fatalln("unable to read confirmation email template " + err.Error())

		}

	}

	if config.ChangeTemplate.Path != nil {

		if config.ChangeTemplate.Email, err = parseFileTemplate(*config.ChangeTemplate.Path); err != nil {

			log.Fatalln("unable to read change email template " + err.Error())

		}

	}

	if config.RecoveryTemplate.Path != nil {

		if config.RecoveryTemplate.Email, err = parseFileTemplate(*config.RecoveryTemplate.Path); err != nil {

			log.Fatalln("unable to read recovery email template", err.Error())

		}

	}

	if config.InvitationTemplate.Path != nil {

		if config.InvitationTemplate.Email, err = parseFileTemplate(*config.InvitationTemplate.Path); err != nil {

			log.Fatalln("unable to read invitation email ", err.Error())

		}

	}

	if config.GoogleEnabled && (config.GoogleClientID == nil || config.GoogleClientSecret == nil) {

		log.Fatalln("expected google_client_id, google_client_secret to be set if google provider is enabled")

	}

	if config.FacebookEnabled && (config.FacebookClientID == nil || config.FacebookClientSecret == nil) {

		log.Fatalln("expected facebook_client_id, facebook_client_secret to be set if facebook provider is enabled")

	}

	if config.GithubEnabled && (config.GithubClientID == nil || config.GithubClientSecret == nil) {

		log.Fatalln("expected github_client_id, github_client_secret to be set if github providder is enabled")

	}

	if config.DisableEmail && config.DisablePhone {

		log.Fatalln("can't disable email and phone at the same time")

	}

	if !config.DisablePhone && config.SMS == nil {

		log.Fatalln("expected sms to be set if phone support is enabled")

	}

	if !config.DisableEmail && config.SMTP == nil {

		log.Fatalln("expected smtp to be set if email support is enabled")

	}

	return &config

}

func (c *Config) GetSigningKey() string {

	if c.JWT.Type == "assymetric" {
		return *c.privateKey
	}

	return *c.JWT.Secret

}

func (c *Config) GetDecodingKey() string {

	if c.JWT.Type == "symmetric" {
		return *c.publicKey
	}

	return *c.JWT.Secret

}

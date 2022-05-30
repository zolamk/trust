package config

import (
	"encoding/json"
	"os"
	"regexp"
	"time"

	"github.com/ohler55/ojg/jp"
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

type JSONPath struct {
	jp.Expr
}

func (p *JSONPath) UnmarshalText(text []byte) error {

	p.Expr = jp.MustParse(text)

	return nil

}

func (p *JSONPath) MarshalText() ([]byte, error) {
	return []byte(p.Expr.String()), nil
}

type TemplateConfig struct {
	Path    string    `json:"path"`
	Subject string    `json:"subject"`
	Email   *Template `json:"-"`
	SMS     *Template `json:"sms"`
}

type SMTPConfig struct {
	Email    string `json:"email"`
	Host     string `json:"host"`
	Password string `json:"password"`
	Port     uint16 `json:"port"`
	Username string `json:"username"`
}

type SMSMapping struct {
	Source      string `json:"source"`
	Destination string `json:"destination"`
	Message     string `json:"message"`
}

type SMSConfig struct {
	URL     string            `json:"url"`
	Method  string            `json:"method"`
	Source  string            `json:"source"`
	Mapping *SMSMapping       `json:"mapping"`
	Headers map[string]string `json:"headers"`
	Extra   map[string]string `json:"extra"`
}

type LockoutPolicy struct {
	Attempts uint8         `json:"attempts"`
	For      time.Duration `json:"for"`
}

type JWTConfig struct {
	Aud            string        `json:"audience"`
	Alg            string        `json:"algorithm"`
	Exp            time.Duration `json:"expiry"`
	Iss            string        `json:"issuer"`
	PrivateKeyPath string        `json:"private_key_path"`
	PublicKeyPath  string        `json:"public_key_path"`
	Secret         string        `json:"secret"`
	Type           string        `json:"-"`
	privateKey     interface{}
	publicKey      interface{}
}

type SocialConfig struct {
	Enabled bool   `json:"enabled"`
	ID      string `json:"id"`
	Secret  string `json:"secret"`
}

type Config struct {
	AccessTokenCookieName     string           `json:"access_token_cookie_name"`
	AccessTokenCookieDomain   string           `json:"access_token_cookie_domain"`
	AdminOnlyList             bool             `json:"admin_only_list"`
	ChangeTemplate            *TemplateConfig  `json:"change_template"`
	ConfirmationTemplate      *TemplateConfig  `json:"confirmation_template"`
	DatabaseURL               string           `json:"database_url"`
	DisableEmail              bool             `json:"disable_email"`
	DisablePhone              bool             `json:"disable_phone"`
	DisableSignup             bool             `json:"disable_signup"`
	EmailRule                 Regexp           `json:"email_rule"`
	Facebook                  SocialConfig     `json:"facebook"`
	Google                    SocialConfig     `json:"google"`
	Github                    SocialConfig     `json:"github"`
	Host                      string           `json:"host"`
	InstanceURL               string           `json:"instance_url"`
	InvitationTemplate        *TemplateConfig  `json:"invitation_template"`
	JWT                       JWTConfig        `json:"jwt"`
	LockoutPolicy             LockoutPolicy    `json:"lockout_policy"`
	LoginHook                 string           `json:"login_hook"`
	LogLevel                  logrus.Level     `json:"log_level"`
	MaxConnectionPoolSize     int              `json:"max_connection_pool_size"`
	MinutesBetweenEmailChange time.Duration    `json:"minutes_between_email_change"`
	MinutesBetweenPhoneChange time.Duration    `json:"minutes_between_phone_change"`
	MinutesBetweenResend      time.Duration    `json:"minutes_between_resend"`
	PasswordHashCost          uint8            `json:"password_hash_cost"`
	PasswordRule              Regexp           `json:"password_rule"`
	PhoneRule                 Regexp           `json:"phone_rule"`
	Port                      uint16           `json:"port"`
	RecoveryTemplate          *TemplateConfig  `json:"recovery_template"`
	RefreshTokenCookieName    string           `json:"refresh_token_cookie_name"`
	RefreshTokenCookieDomain  string           `json:"refresh_token_cookie_domain"`
	SetAccessTokenCookie      bool             `json:"set_access_token_cookie"`
	SetRefreshTokenCookie     bool             `json:"set_refresh_token_cookie"`
	SiteURL                   string           `json:"site_url"`
	SMS                       *SMSConfig       `json:"sms"`
	SMTP                      *SMTPConfig      `json:"smtp"`
	SocialRedirectPage        string           `json:"social_redirect_page"`
	CustomDataSchema          map[string]Field `json:"custom_data_schema"`
	MetadataPath              *JSONPath        `json:"metadata_path"`
	AdminRoles                []string         `json:"admin_roles"`
	ReadOnlyRoles             []string         `json:"read_only_roles"`
	RolesPath                 JSONPath         `json:"roles_path"`
}

func NewDefaultConfig() *Config {

	defaultConfirmationEmailTemplate, _ := parseStringTemplate(`
	<h2>Confirm Your Email Address</h2>
	<p>Follow this link to confirm your email</p>
	<p>
		<a href='{{ site_url }}?token={{ email_confirmation_token }}'>Confirm</a>
	</p>
	`)

	defaultInvitationEmailTemplate, _ := parseStringTemplate(`
	<h2>You Have Been Invited</h2>
	<p>Follow this link to accept your invitation</p>
	<p>
		<a href='{{ site_url }}?token={{ email_invitation_token }}'>Accept Invite</a>
	</p>
	`)

	defaultRecoveryEmailTemplate, _ := parseStringTemplate(`
	<h2>Recover Your Account</h2>
	<p>Follow this link to recover you account</p>
	<p>
		<a href='{{ site_url }}?token={{ email_recovery_token }}'>Recover</a>
	</p>`)

	defaultChangeEmailTemplate, _ := parseStringTemplate(`
	<h2>Change Your Email Address</h2>
	<p>Follow this link to confirm your email address change</p>
	<p>
		<a href='{{ site_url }}?token={{ email_change_token }}'>Confirm</a>
	</p>
	`)

	defaultConfirmationSMSTemplate, _ := parseStringTemplate("Phone Confirmation Code - {{ phone_confirmation_token }}")

	defaultInvitationSMSTemplate, _ := parseStringTemplate("Phone Invitation Code - {{ phone_invitation_token }}")

	defaultRecoverySMSTemplate, _ := parseStringTemplate("Phone Recovery Code - {{ phone_recovery_token }}")

	defaultChangeSMSTemplate, _ := parseStringTemplate("Phone Change Code -  {{ phone_change_token }}")

	return &Config{
		AdminRoles:    []string{"trust:admin"},
		ReadOnlyRoles: []string{"trust:read"},
		DisableSignup: false,
		Facebook: SocialConfig{
			Enabled: false,
		},
		Google: SocialConfig{
			Enabled: false,
		},
		Github: SocialConfig{
			Enabled: false,
		},
		Host: "localhost",
		JWT: JWTConfig{
			Exp:  900,
			Type: "asymmetric",
			Aud:  "trust",
			Iss:  "trust",
		},
		LogLevel:                  logrus.InfoLevel,
		Port:                      1995,
		DisablePhone:              false,
		DisableEmail:              false,
		AdminOnlyList:             true,
		SocialRedirectPage:        "social",
		PasswordHashCost:          10,
		MaxConnectionPoolSize:     10,
		MinutesBetweenResend:      10,
		MinutesBetweenPhoneChange: 1440,
		MinutesBetweenEmailChange: 1440,
		PasswordRule:              Regexp{*regexp.MustCompile(".{8,1000}")},
		EmailRule:                 Regexp{*regexp.MustCompile(`^[\w\-\.]+@([\w\-]+\.)+[\w\-]{1,}$`)},
		PhoneRule:                 Regexp{*regexp.MustCompile(`\+\d{5,15}`)},
		RefreshTokenCookieName:    "trust_refresh_token",
		RolesPath: JSONPath{
			jp.MustParse([]byte("$.roles")),
		},
		SetAccessTokenCookie:  true,
		SetRefreshTokenCookie: true,
		AccessTokenCookieName: "trust_access_token",
		LockoutPolicy: LockoutPolicy{
			Attempts: 5,
			For:      60,
		},
		ConfirmationTemplate: &TemplateConfig{
			Subject: "Confirm Your Account",
			SMS:     defaultConfirmationSMSTemplate,
			Email:   defaultConfirmationEmailTemplate,
		},
		InvitationTemplate: &TemplateConfig{
			SMS:     defaultInvitationSMSTemplate,
			Subject: "You've Been Invited",
			Email:   defaultInvitationEmailTemplate,
		},
		RecoveryTemplate: &TemplateConfig{
			SMS:     defaultRecoverySMSTemplate,
			Subject: "Recover Your Account",
			Email:   defaultRecoveryEmailTemplate,
		},
		ChangeTemplate: &TemplateConfig{
			SMS:     defaultChangeSMSTemplate,
			Subject: "Confirm Email Change",
			Email:   defaultChangeEmailTemplate,
		},
	}

}

func validateAsymmetric(config *Config) error {

	if config.JWT.PrivateKeyPath == "" || config.JWT.PublicKeyPath == "" {
		return ErrAsymmetricKeyPathsNotSet
	}

	switch config.JWT.Alg {
	case "RS256", "RS384", "RS512":

		private_key, err := parsePKCS8PrivateKey(config.JWT.PrivateKeyPath)

		if err != nil {

			return ErrParsingPrivateKey

		}

		config.JWT.privateKey = private_key

		public_key, err := parsePKIXPublicKey(config.JWT.PublicKeyPath)

		if err != nil {

			return ErrParsingPublicKey

		}

		config.JWT.publicKey = public_key

	case "ES256", "ES384", "ES512":

		private_key, err := parseECPrivateKey(config.JWT.PrivateKeyPath)

		if err != nil {

			return ErrParsingPrivateKey

		}

		config.JWT.privateKey = private_key

		public_key, err := parsePKIXPublicKey(config.JWT.PublicKeyPath)

		if err != nil {

			return ErrParsingPublicKey

		}

		config.JWT.publicKey = public_key

	default:
		return ErrUnsupportedAlgorithm

	}

	return nil

}

func validateKeys(config *Config) error {
	switch config.JWT.Alg {
	case "RS256", "RS384", "RS512", "ES256", "ES384", "ES512":

		return validateAsymmetric(config)

	case "HS256", "HS384", "HS512":

		if config.JWT.Secret == "" {
			return ErrSymmetricSecretNotSet
		}

		config.JWT.Type = "symmetric"

	default:
		return ErrUnsupportedAlgorithm
	}

	return nil

}

func validateTemplates(config *Config) error {

	var err error

	if config.ConfirmationTemplate.Path != "" {

		if config.ConfirmationTemplate.Email, err = parseFileTemplate(config.ConfirmationTemplate.Path); err != nil {

			return ErrUnableToReadTemplate

		}

	}

	if config.ChangeTemplate.Path != "" {

		if config.ChangeTemplate.Email, err = parseFileTemplate(config.ChangeTemplate.Path); err != nil {

			return ErrUnableToReadTemplate

		}

	}

	if config.RecoveryTemplate.Path != "" {

		if config.RecoveryTemplate.Email, err = parseFileTemplate(config.RecoveryTemplate.Path); err != nil {

			return ErrUnableToReadTemplate

		}

	}

	if config.InvitationTemplate.Path != "" {

		if config.InvitationTemplate.Email, err = parseFileTemplate(config.InvitationTemplate.Path); err != nil {

			return ErrUnableToReadTemplate

		}

	}

	return nil

}

func validateSymbol(config *Config) error {

	if config.Google.Enabled && (config.Google.ID == "" || config.Google.Secret == "") {

		return ErrGoogleConfig

	}

	if config.Facebook.Enabled && (config.Facebook.ID == "" || config.Facebook.Secret == "") {

		return ErrFacebookConfig

	}

	if config.Github.Enabled && (config.Github.ID == "" || config.Github.Secret == "") {

		return ErrGithubConfig

	}

	return nil

}

func validateOther(config *Config) error {

	if config.DisableEmail && config.DisablePhone {

		return ErrPhoneEmailDisabled

	}

	if !config.DisablePhone && config.SMS == nil {

		return ErrSMSNotConfigured

	}

	if !config.DisableEmail && config.SMTP == nil {

		return ErrEmailNotConfigured

	}

	return nil

}

func New(path string) (*Config, error) {

	config := NewDefaultConfig()

	file, err := os.ReadFile(path)

	if err != nil {
		return nil, err
	}

	if err = json.Unmarshal(file, &config); err != nil {
		return nil, err
	}

	if err = validateKeys(config); err != nil {
		return nil, err
	}

	if err = validateTemplates(config); err != nil {
		return nil, err
	}

	if err = validateSymbol(config); err != nil {
		return nil, err
	}

	if err = validateOther(config); err != nil {
		return nil, err
	}

	return config, nil

}

func (c *JWTConfig) GetSigningKey() interface{} {

	if c.Type == "asymmetric" {
		return c.privateKey
	}

	return []byte(c.Secret)

}

func (c *JWTConfig) GetDecodingKey() interface{} {

	if c.Type == "asymmetric" {
		return c.publicKey
	}

	return []byte(c.Secret)

}

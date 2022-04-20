package middleware

type contextKey string

const (
	TokenKey        = contextKey("token")
	WriterKey       = contextKey("writer")
	LogDataKey      = contextKey("log_data")
	ProviderKey     = contextKey("provider")
	RefreshTokenKey = contextKey("refresh_token")
)

package middleware

type contextKey struct {
	name string
}

var TokenKey = &contextKey{"token"}
var WriterKey = &contextKey{"writer"}
var RefreshTokenKey = &contextKey{"refresh_token"}
var ProviderKey = &contextKey{"provider"}
var IPKey = &contextKey{"ip_key"}
var UserAgentKey = &contextKey{"user_agent"}

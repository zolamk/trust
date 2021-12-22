package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"
	"net/http"

	"github.com/99designs/gqlgen/graphql"
	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/handlers/anonymous"
	"github.com/zolamk/trust/handlers/user"
	"github.com/zolamk/trust/handlers/users"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
)

func (r *queryResolver) User(ctx context.Context, id string) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	return users.User(r.DB, r.Config, token, id)
}

func (r *queryResolver) Users(ctx context.Context, where map[string]interface{}, orderBy map[string]interface{}, offset int, limit int) ([]*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	fields := graphql.CollectAllFields(ctx)

	return users.Users(r.DB, r.Config, token, fields, where, orderBy, offset, limit)
}

func (r *queryResolver) Me(ctx context.Context) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	return user.Me(r.DB, r.Config, token)
}

func (r *queryResolver) Token(ctx context.Context, username string, password string) (*model.LoginResponse, error) {

	writer := ctx.Value(middleware.WriterKey).(http.ResponseWriter)

	ip := ctx.Value(middleware.IPKey).(string)

	user_agent := ctx.Value(middleware.UserAgentKey).(string)

	return anonymous.Token(r.DB, r.Config, r.IP2LocationDB, username, password, writer, ip, user_agent)

}

func (r *queryResolver) Refresh(ctx context.Context) (*model.LoginResponse, error) {
	writer := ctx.Value(middleware.WriterKey).(http.ResponseWriter)

	refresh_token := ctx.Value(middleware.RefreshTokenKey).(string)

	provider := ctx.Value(middleware.ProviderKey).(string)

	return anonymous.RefreshToken(r.DB, r.Config, refresh_token, provider, writer)
}

func (r *queryResolver) AuditLogs(ctx context.Context) ([]*model.Log, error) {
	panic(fmt.Errorf("not implemented"))
}

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type queryResolver struct{ *Resolver }

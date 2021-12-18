package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers/lib"
	"github.com/zolamk/trust/handlers/lib/user"
	"github.com/zolamk/trust/handlers/lib/users"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
)

func (r *queryResolver) User(ctx context.Context, id string) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *queryResolver) Users(ctx context.Context, limit int, offset int) ([]*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return users.Users(r.DB, r.Config, token, limit, offset)
}

func (r *queryResolver) Me(ctx context.Context) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return user.Me(r.DB, r.Config, token)
}

func (r *queryResolver) Token(ctx context.Context, username string, password string) (*model.LoginResponse, error) {
	return lib.Token(r.DB, r.Config, username, password)
}

func (r *queryResolver) Refresh(ctx context.Context, token string) (*model.LoginResponse, error) {
	return lib.RefreshToken(r.DB, r.Config, token)
}

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type queryResolver struct{ *Resolver }

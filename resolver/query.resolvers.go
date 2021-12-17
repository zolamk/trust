package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers/lib"
	"github.com/zolamk/trust/model"
)

func (r *queryResolver) User(ctx context.Context, id string) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *queryResolver) Users(ctx context.Context, limit int, offset int) ([]*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *queryResolver) Me(ctx context.Context) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *queryResolver) Token(ctx context.Context, username string, password string) (*model.LoginResponse, error) {
	return lib.Token(r.DB, r.Config, username, password)
}

func (r *queryResolver) Refresh(ctx context.Context, token string) (*model.LoginResponse, error) {
	panic(fmt.Errorf("not implemented"))
}

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type queryResolver struct{ *Resolver }

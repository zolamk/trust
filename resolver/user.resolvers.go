package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/model"
)

func (r *userResolver) EmailConfirmationTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) EmailConfirmedAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) PhoneConfirmationTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) PhoneConfirmedAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) RecoveryTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) EmailChangeTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) PhoneChangeTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) LastSigninAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) CreatedAt(ctx context.Context, obj *model.User) (string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) UpdatedAt(ctx context.Context, obj *model.User) (string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) InvitationTokenSentAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *userResolver) InvitationAcceptedAt(ctx context.Context, obj *model.User) (*string, error) {
	panic(fmt.Errorf("not implemented"))
}

// User returns generated.UserResolver implementation.
func (r *Resolver) User() generated.UserResolver { return &userResolver{r} }

type userResolver struct{ *Resolver }

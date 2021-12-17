package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/zolamk/trust/errors"
	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers/lib"
	"github.com/zolamk/trust/handlers/lib/reset"
	"github.com/zolamk/trust/handlers/lib/user"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/model"
)

func (r *mutationResolver) Signup(ctx context.Context, object model.SignupForm) (*model.User, error) {
	return lib.Signup(r.DB, r.Config, object)
}

func (r *mutationResolver) ConfirmEmail(ctx context.Context, token string) (*model.User, error) {
	return lib.ConfirmEmail(r.DB, r.Config, token)
}

func (r *mutationResolver) ConfirmPhone(ctx context.Context, token string) (*model.User, error) {
	return lib.ConfirmPhone(r.DB, r.Config, token)
}

func (r *mutationResolver) InviteUser(ctx context.Context, object model.InviteForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) AcceptInvite(ctx context.Context, object model.AcceptInviteForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) CreateUser(ctx context.Context, object model.CreateUserForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) UpdateUser(ctx context.Context, id string, object model.UpdateUserForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) DeleteUser(ctx context.Context, id string) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) UpdateEmail(ctx context.Context, id string, object model.UpdateEmailForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) UpdatePhone(ctx context.Context, id string, object model.UpdatePhoneForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) UpdatePassword(ctx context.Context, id string, object model.UpdatePasswordForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ChangePassword(ctx context.Context, object model.ChangePasswordForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ChangeEmail(ctx context.Context, email string) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return user.ChangeEmail(r.DB, r.Config, token, email)
}

func (r *mutationResolver) ChangePhone(ctx context.Context, phone string) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return user.ChangePhone(r.DB, r.Config, token, phone)
}

func (r *mutationResolver) ConfirmPhoneChange(ctx context.Context, confirmation_token string) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return user.ConfirmPhoneChange(r.DB, r.Config, token, confirmation_token)
}

func (r *mutationResolver) ConfirmEmailChange(ctx context.Context, confirmation_token string) (*model.User, error) {
	token, ok := ctx.Value("token").(*jwt.JWT)

	if !ok {
		return nil, errors.ErrInvalidJWT
	}

	return user.ConfirmEmailChange(r.DB, r.Config, token, confirmation_token)
}

func (r *mutationResolver) Reset(ctx context.Context, username string) (bool, error) {
	return reset.Reset(r.DB, r.Config, username)
}

func (r *mutationResolver) ConfirmReset(ctx context.Context, recoveryToken string, password string) (bool, error) {
	return reset.ConfirmReset(r.DB, r.Config, recoveryToken, password)
}

func (r *mutationResolver) ResendPhoneConfirmation(ctx context.Context, phone string) (bool, error) {
	return lib.ResendPhone(r.DB, r.Config, phone)
}

func (r *mutationResolver) ResendEmailConfirmation(ctx context.Context, email string) (bool, error) {
	return lib.ResendEmail(r.DB, r.Config, email)
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

type mutationResolver struct{ *Resolver }

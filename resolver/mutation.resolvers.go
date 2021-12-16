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

func (r *mutationResolver) Signup(ctx context.Context, object model.SignupForm) (*model.User, error) {
	return lib.Signup(r.DB, r.Config, object)
}

func (r *mutationResolver) ConfirmEmail(ctx context.Context, token string) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ConfirmPhone(ctx context.Context, token string) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
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

func (r *mutationResolver) ChangeEmail(ctx context.Context, object model.ChangeEmailForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ChangePhone(ctx context.Context, object model.ChangePhoneForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ConfirmPhoneChange(ctx context.Context, object model.ConfirmPhoneChangeForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ConfirmEmailChange(ctx context.Context, object model.ConfirmChangeEmailForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) Reset(ctx context.Context, object model.ResetForm) (bool, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ConfirmReset(ctx context.Context, object model.ConfirmResetForm) (*model.User, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ResendPhoneConfirmation(ctx context.Context, object model.ResendPhoneForm) (bool, error) {
	panic(fmt.Errorf("not implemented"))
}

func (r *mutationResolver) ResendEmailConfirmation(ctx context.Context, object model.ResendEmailForm) (bool, error) {
	panic(fmt.Errorf("not implemented"))
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

type mutationResolver struct{ *Resolver }
package resolver

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"

	"github.com/zolamk/trust/graph/generated"
	"github.com/zolamk/trust/handlers"
	"github.com/zolamk/trust/handlers/anonymous"
	"github.com/zolamk/trust/handlers/reset"
	"github.com/zolamk/trust/handlers/user"
	"github.com/zolamk/trust/handlers/users"
	"github.com/zolamk/trust/handlers/users/update"
	"github.com/zolamk/trust/jwt"
	"github.com/zolamk/trust/middleware"
	"github.com/zolamk/trust/model"
)

func (r *mutationResolver) Signup(ctx context.Context, object model.SignupForm) (*model.User, error) {
	return anonymous.Signup(r.DB, r.Config, object)
}

func (r *mutationResolver) ConfirmEmail(ctx context.Context, token string) (*model.User, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return anonymous.ConfirmEmail(r.DB, r.Config, token, &log_data)

}

func (r *mutationResolver) ConfirmPhone(ctx context.Context, token string) (*model.User, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return anonymous.ConfirmPhone(r.DB, r.Config, token, &log_data)
}

func (r *mutationResolver) InviteByEmail(ctx context.Context, name string, email string) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return users.InviteEmail(r.DB, r.Config, jwt, name, email, &log_data)

}

func (r *mutationResolver) InviteByPhone(ctx context.Context, name string, phone string) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return users.InvitePhone(r.DB, r.Config, jwt, name, phone, &log_data)

}

func (r *mutationResolver) AcceptPhoneInvite(ctx context.Context, token string, password string) (*model.User, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return anonymous.AcceptPhoneInvite(r.DB, r.Config, token, password, &log_data)

}

func (r *mutationResolver) AcceptEmailInvite(ctx context.Context, token string, password string) (*model.User, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return anonymous.AcceptEmailInvite(r.DB, r.Config, token, password, &log_data)

}

func (r *mutationResolver) CreateUser(ctx context.Context, object model.CreateUserForm) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return users.CreateUser(r.DB, r.Config, jwt, object, &log_data)

}

func (r *mutationResolver) UpdateUser(ctx context.Context, id string, name *string, avatar *string) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	return update.UpdateUser(r.DB, r.Config, jwt, id, name, avatar)

}

func (r *mutationResolver) DeleteUser(ctx context.Context, id string) (*model.User, error) {
	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	return users.DeleteUser(r.DB, r.Config, jwt, id)
}

func (r *mutationResolver) UpdateEmail(ctx context.Context, id string, email string, confirm *bool) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return update.UpdateEmail(r.DB, r.Config, jwt, id, email, confirm, &log_data)

}

func (r *mutationResolver) UpdatePhone(ctx context.Context, id string, phone string, confirm *bool) (*model.User, error) {
	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return update.UpdatePhone(r.DB, r.Config, jwt, id, phone, confirm, &log_data)
}

func (r *mutationResolver) UpdatePassword(ctx context.Context, id string, password string) (*model.User, error) {
	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return update.UpdatePassword(r.DB, r.Config, jwt, id, password, &log_data)
}

func (r *mutationResolver) ChangePassword(ctx context.Context, oldPassword string, newPassword string) (*model.User, error) {

	token, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return user.ChangePassword(r.DB, r.Config, token, oldPassword, newPassword, &log_data)

}

func (r *mutationResolver) ChangeEmail(ctx context.Context, email string) (*model.User, error) {

	token, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return user.ChangeEmail(r.DB, r.Config, token, email, &log_data)

}

func (r *mutationResolver) ChangePhone(ctx context.Context, phone string) (*model.User, error) {

	token, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return user.ChangePhone(r.DB, r.Config, token, phone, &log_data)

}

func (r *mutationResolver) ConfirmPhoneChange(ctx context.Context, token string) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return user.ConfirmPhoneChange(r.DB, r.Config, jwt, token, &log_data)

}

func (r *mutationResolver) ConfirmEmailChange(ctx context.Context, token string) (*model.User, error) {

	jwt, ok := ctx.Value(middleware.TokenKey).(*jwt.JWT)

	if !ok {
		return nil, handlers.ErrInvalidJWT
	}

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return user.ConfirmEmailChange(r.DB, r.Config, jwt, token, &log_data)

}

func (r *mutationResolver) Reset(ctx context.Context, username string) (bool, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return reset.Reset(r.DB, r.Config, username, &log_data)

}

func (r *mutationResolver) ConfirmReset(ctx context.Context, token string, password string) (bool, error) {

	log_data := ctx.Value(middleware.LogDataKey).(middleware.LogData)

	return reset.ConfirmReset(r.DB, r.Config, token, password, &log_data)

}

func (r *mutationResolver) ResendPhoneConfirmation(ctx context.Context, phone string) (bool, error) {
	return anonymous.ResendPhone(r.DB, r.Config, phone)
}

func (r *mutationResolver) ResendEmailConfirmation(ctx context.Context, email string) (bool, error) {
	return anonymous.ResendEmail(r.DB, r.Config, email)
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

type mutationResolver struct{ *Resolver }

// Code generated by github.com/99designs/gqlgen, DO NOT EDIT.

package model

type ConfirmResetForm struct {
	RecoveryToken string `json:"recovery_token"`
	NewPassword   string `json:"new_password"`
}

type ResetForm struct {
	Username string `json:"username"`
}

type AcceptInviteForm struct {
	InvitationToken string `json:"invitation_token"`
	Password        string `json:"password"`
}

type ChangeEmailForm struct {
	Email string `json:"email"`
}

type ChangePasswordForm struct {
	OldPassword string `json:"old_password"`
	NewPassword string `json:"new_password"`
}

type ChangePhoneForm struct {
	Phone string `json:"phone"`
}

type ConfirmChangeEmailForm struct {
	EmailChangeToken string `json:"email_change_token"`
}

type ConfirmPhoneChangeForm struct {
	PhoneChangeToken string `json:"phone_change_token"`
}

type CreateUserForm struct {
	Email    *string `json:"email"`
	Phone    *string `json:"phone"`
	Password *string `json:"password"`
	Name     *string `json:"name"`
	Avatar   *string `json:"avatar"`
	Confirm  *bool   `json:"confirm"`
}

type InviteForm struct {
	Name  *string `json:"name"`
	Email *string `json:"email"`
	Phone *string `json:"phone"`
}

type LoginResponse struct {
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
	ID           string `json:"id"`
}

type SignupForm struct {
	Name     *string `json:"name"`
	Avatar   *string `json:"avatar"`
	Email    *string `json:"email"`
	Phone    *string `json:"phone"`
	Password string  `json:"password"`
}

type UpdateEmailForm struct {
	Email   string `json:"email"`
	Confirm *bool  `json:"confirm"`
}

type UpdatePasswordForm struct {
	Password string `json:"password"`
}

type UpdatePhoneForm struct {
	Phone   string `json:"phone"`
	Confirm *bool  `json:"confirm"`
}

type UpdateUserForm struct {
	Name   *string `json:"name"`
	Avatar *string `json:"avatar"`
}

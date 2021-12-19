package handlers

import (
	"github.com/vektah/gqlparser/v2/gqlerror"
)

var (
	ErrInternal = &gqlerror.Error{
		Message: "internal error server",
		Extensions: map[string]interface{}{
			"code": "internal",
		},
	}
	ErrTooManyRequests = &gqlerror.Error{
		Message: "too many requests",
		Extensions: map[string]interface{}{
			"code": "too_many_requests",
		},
	}
	ErrIncorrectUsernameOrPassword = &gqlerror.Error{
		Message: "incorrect username or password",
		Extensions: map[string]interface{}{
			"code": "incorrect_username_or_password",
		},
	}
	ErrEmailNotConfirmed = &gqlerror.Error{
		Message: "email not confirmed",
		Extensions: map[string]interface{}{
			"code": "email_not_confirmed",
		},
	}
	ErrPhoneNotConfirmed = &gqlerror.Error{
		Message: "phone not confirmed",
		Extensions: map[string]interface{}{
			"code": "phone_not_confirmed",
		},
	}
	ErrWebHook = &gqlerror.Error{
		Message: "webhook error",
		Extensions: map[string]interface{}{
			"code": "webhook_error",
		},
	}
	ErrRefreshTokenNotFound = &gqlerror.Error{
		Message: "refresh token not found",
		Extensions: map[string]interface{}{
			"code": "refresh_token_not_found",
		},
	}
	ErrInvalidEmail = &gqlerror.Error{
		Message: "invalid email address",
		Extensions: map[string]interface{}{
			"code": "invalid_email_address",
		},
	}
	ErrInvalidPhone = &gqlerror.Error{
		Message: "invalid phone number",
		Extensions: map[string]interface{}{
			"code": "invalid_phone_number",
		},
	}
	ErrInvalidPassword = &gqlerror.Error{
		Message: "invalid password",
		Extensions: map[string]interface{}{
			"code": "invalid_password",
		},
	}
	ErrEmailDisabled = &gqlerror.Error{
		Message: "email disabled",
		Extensions: map[string]interface{}{
			"code": "email_disabled",
		},
	}
	ErrPhoneDisabled = &gqlerror.Error{
		Message: "phone disabled",
		Extensions: map[string]interface{}{
			"code": "phone_disabled",
		},
	}
	ErrEmailRegistered = &gqlerror.Error{
		Message: "email address registered",
		Extensions: map[string]interface{}{
			"code": "email_address_registered",
		},
	}
	ErrPhoneRegistered = &gqlerror.Error{
		Message: "phone number registered",
		Extensions: map[string]interface{}{
			"code": "phone_number_registered",
		},
	}
	ErrRecoveryTokenNotFound = &gqlerror.Error{
		Message: "recovery token not found",
		Extensions: map[string]interface{}{
			"code": "recovery_token_not_found",
		},
	}
	ErrInvalidJWT = &gqlerror.Error{
		Message: "invalid json web token",
		Extensions: map[string]interface{}{
			"code": "invalid_jwt",
		},
	}
	ErrUserNotFound = &gqlerror.Error{
		Message: "user not found",
		Extensions: map[string]interface{}{
			"code": "user_not_found",
		},
	}
	ErrNewPhoneSimilar = &gqlerror.Error{
		Message: "new phone must be different from old phone",
		Extensions: map[string]interface{}{
			"code": "new_phone_equals_old",
		},
	}
	ErrNewEmailSimilar = &gqlerror.Error{
		Message: "new email must be different from old email",
		Extensions: map[string]interface{}{
			"code": "new_email_equals_old",
		},
	}
	ErrCantChangePhoneNow = &gqlerror.Error{
		Message: "cant change phone right now",
		Extensions: map[string]interface{}{
			"code": "cant_change_phone",
		},
	}
	ErrCantChangeEmailNow = &gqlerror.Error{
		Message: "cant change email right now",
		Extensions: map[string]interface{}{
			"code": "cant_change_email",
		},
	}
	ErrIncorrectOldPassword = &gqlerror.Error{
		Message: "incorrect old password",
		Extensions: map[string]interface{}{
			"code": "incorrect_old_password",
		},
	}
	ErrAdminOnly = &gqlerror.Error{
		Message: "only an admin can perform this action",
		Extensions: map[string]interface{}{
			"code": "admin_only",
		},
	}
	ErrEmailOrPhoneRequired = &gqlerror.Error{
		Message: "email or phone required",
		Extensions: map[string]interface{}{
			"code": "email_or_phone_required",
		},
	}
	ErrCantChangeOwnAccount = &gqlerror.Error{
		Message: "can't change your own account",
		Extensions: map[string]interface{}{
			"code": "cant_change_own_account",
		},
	}
)

use crate::{
    handlers::{
        graphql::context::Context,
        lib::{
            accept_invite, confirm_email, confirm_phone,
            reset::{confirm_reset, reset},
            resend_phone,
            resend_email,
            signup,
            user::{change_email, change_email_confirm, change_password, change_phone, change_phone_confirm},
            users::{
                create, delete, invite,
                update::{email, password, phone, update},
            },
        },
        Error as HandlerError,
    },
    models::user::User,
};

#[derive(Debug)]
pub struct Mutation {}

#[derive(GraphQLObject)]
struct ResetResponse {
    pub accepted: bool,
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn signup(context: &Context, object: signup::SignUpForm) -> Result<User, HandlerError> {
        let user = signup::signup(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_email")]
    fn confirm_email(context: &Context, token: String) -> Result<User, HandlerError> {
        let user = confirm_email::confirm(&context.connection, confirm_email::ConfirmForm { confirmation_token: token });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_phone")]
    fn confirm_phone(context: &Context, token: String) -> Result<User, HandlerError> {
        let user = confirm_phone::confirm(&context.connection, confirm_phone::ConfirmForm { confirmation_token: token });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "invite_user")]
    fn invite_user(context: &Context, object: invite::InviteForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = invite::invite(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "accept_invite")]
    fn accept_invite(context: &Context, object: accept_invite::AcceptForm) -> Result<User, HandlerError> {
        let user = accept_invite::accept_invite(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "create_user")]
    fn create_user(context: &Context, object: create::CreateForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = create::create(&context.config, &context.connection, token, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_user")]
    fn update_user(context: &Context, id: String, object: update::UpdateForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = update::update(&context.connection, token, object, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "delete_user")]
    fn delete_user(context: &Context, id: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = delete::delete(&context.connection, token, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_email")]
    fn update_email(context: &Context, id: String, object: email::UpdateEmailForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = email::update_email(&context.config, &context.connection, token, object, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_phone")]
    fn update_phone(context: &Context, id: String, object: phone::UpdatePhoneForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = phone::update_phone(&context.config, &context.connection, token, object, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_password")]
    fn update_password(context: &Context, id: String, object: password::UpdatePasswordForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = password::update_password(&context.config, &context.connection, token, object, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_password")]
    fn change_password(context: &Context, object: change_password::ChangePasswordForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_password::change_password(&context.config, &context.connection, token, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_email")]
    fn change_email(context: &Context, object: change_email::ChangeEmailForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_email::change_email(&context.config, &context.connection, token, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_phone")]
    fn change_phone(context: &Context, object: change_phone::ChangePhoneForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_phone::change_phone(&context.config, &context.connection, token, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_phone_change")]
    fn confirm_phone_change(context: &Context, object: change_phone_confirm::ConfirmPhoneChangeForm) -> Result<User, HandlerError> {
        let user = change_phone_confirm::change_phone_confirm(&context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_email_change")]
    fn confirm_email_change(context: &Context, object: change_email_confirm::ConfirmChangeEmailForm) -> Result<User, HandlerError> {
        let user = change_email_confirm::change_email_confirm(&context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    fn reset(context: &Context, object: reset::ResetForm) -> Result<bool, HandlerError> {
        let reset = reset::reset(&context.config, &context.connection, object);

        if reset.is_err() {
            return Err(reset.err().unwrap());
        }

        return Ok(true);
    }

    #[graphql(name = "confirm_reset")]
    fn confirm_reset(context: &Context, object: confirm_reset::ConfirmResetForm) -> Result<User, HandlerError> {
        let user = confirm_reset::confirm_reset(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "resend_phone_confirmation")]
    fn resend_phone(context: &Context, object: resend_phone::ResendPhoneForm) -> Result<bool, HandlerError> {
        let user = resend_phone::resend_phone(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(true);
    }

    #[graphql(name = "resend_email_confirmation")]
    fn resend_email(context: &Context, object: resend_email::ResendEmailForm) -> Result<bool, HandlerError> {
        let user = resend_email::resend_email(&context.config, &context.connection, object);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(true);
    }
}

use crate::{
    handlers::{
        graphql::context::Context,
        lib::{
            confirm_email, confirm_phone,
            reset::{confirm_reset, reset},
            signup,
            user::{change_email, change_email_confirm, change_password, change_phone, change_phone_confirm},
            users::{
                create, delete,
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
    fn signup(context: &Context, user: signup::SignUpForm) -> Result<User, HandlerError> {
        let user = signup::signup(&context.config, &context.connection, context.operator_signature.clone(), user);

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

    #[graphql(name = "create_user")]
    fn create_user(context: &Context, user: create::CreateForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = create::create(&context.config, &context.connection, token, user);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_user")]
    fn update_user(context: &Context, id: String, user: update::UpdateForm) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = update::update(&context.connection, token, user, id);

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
    fn update_email(context: &Context, id: String, confirm: Option<bool>, email: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = email::update_email(&context.config, &context.connection, token, email::UpdateForm { email, confirm }, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_phone")]
    fn update_phone(context: &Context, id: String, confirm: Option<bool>, phone: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = phone::update_phone(&context.config, &context.connection, token, phone::UpdateForm { phone, confirm }, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "update_password")]
    fn update_password(context: &Context, id: String, password: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = password::update_password(&context.config, &context.connection, token, password::UpdateForm { password }, id);

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_password")]
    fn change_password(context: &Context, old_password: String, new_password: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_password::change_password(&context.config, &context.connection, token, change_password::ChangePasswordForm { old_password, new_password });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_email")]
    fn change_email(context: &Context, email: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_email::change_email(&context.config, &context.connection, token, change_email::ChangeEmailFrom { email });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "change_phone")]
    fn change_phone(context: &Context, phone: String) -> Result<User, HandlerError> {
        let token = context.token.as_ref();

        if token.is_err() {
            let err = token.err().unwrap();

            return Err(HandlerError::from(err));
        }

        let token = token.unwrap();

        let user = change_phone::change_phone(&context.config, &context.connection, token, change_phone::ChangePhoneForm { phone });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_phone_change")]
    fn confirm_phone_change(context: &Context, token: String) -> Result<User, HandlerError> {
        let user = change_phone_confirm::change_phone_confirm(&context.connection, change_phone_confirm::ConfirmPhoneChangeForm { phone_change_token: token });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    #[graphql(name = "confirm_email_change")]
    fn confirm_email_change(context: &Context, token: String) -> Result<User, HandlerError> {
        let user = change_email_confirm::change_email_confirm(&context.connection, change_email_confirm::ConfirmChangeEmailForm { email_change_token: token });

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }

    fn reset(context: &Context, username: String) -> Result<ResetResponse, HandlerError> {
        let reset = reset::reset(&context.config, &context.connection, reset::ResetForm { username });

        if reset.is_err() {
            return Err(reset.err().unwrap());
        }

        return Ok(ResetResponse { accepted: true });
    }

    #[graphql(name = "confirm_reset")]
    fn confirm_reset(context: &Context, token: String, password: String) -> Result<User, HandlerError> {
        let user = confirm_reset::confirm_reset(
            &context.config,
            &context.connection,
            confirm_reset::ConfirmResetForm {
                recovery_token: token,
                new_password: password,
            },
        );

        if user.is_err() {
            return Err(user.err().unwrap());
        }

        return Ok(user.unwrap());
    }
}

//! src/routes/admin/password/post.rs
use crate::authentication::{AuthError, Credentials, validate_credentials};
use crate::routes::admin::dashboard::get_username;

use crate::utils::{e500, see_other};

use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use sqlx::PgPool;

use crate::authentication::UserId;
use secrecy::ExposeSecret;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}
pub async fn change_password(
    form: web::Form<FormData>,
    // No longer injecting TypedSession!
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    // `Secret<String>` does not implement `Eq`,
    // therefore we need to compare the underlying `String`.
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    if form.new_password.expose_secret().len() < 12 {
        FlashMessage::error("Your password must be at least 12 characters long.").send();

        return Ok(see_other("/admin/password"));
    }

    if form.new_password.expose_secret().len() > 128 {
        FlashMessage::error("Your password must be shorter than 129 characters.").send();

        return Ok(see_other("/admin/password"));
    }
    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        };
    }
    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;
    FlashMessage::error("Your password has been changed.").send();
    Ok(see_other("/admin/password"))
}

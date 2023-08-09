use actix_web::{HttpRequest, HttpResponse, web};
use actix_identity::Identity;
use actix_http::header::LOCATION;
use actix_web::http::header::ContentType;
use magic_crypt::MagicCryptTrait;
use actix_http::HttpMessage;
use crate::controller::common::all_structs::User;
use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginCheck};

pub async fn get_data_from_login_page(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
    let mcrypt = &config.magic_key;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    let db = &config.database_connection;
    let result = login_database(username, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let y = LoginCheck { value: 1 };
    if result == y {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .content_type(ContentType::html())
            .finish())
    }
}

use async_trait::async_trait;
use pgwire::api::auth::{AuthSource, LoginInfo, Password};
use pgwire::error::{ErrorInfo, PgWireResult};
use pgwire::api::auth::md5pass::hash_md5_password;
use pgwire::error::PgWireError;

use crate::config::FATHERDUCK_CONFIG;

pub struct FatherDuckAuthSource;

#[async_trait]
impl AuthSource for FatherDuckAuthSource {
    async fn get_password(&self, login_info: &LoginInfo) -> PgWireResult<Password> {
        println!("login info: {:?}", login_info);

        match login_info.user() {
            Some(username) => {
                if username != FATHERDUCK_CONFIG.username {
                    return Err(PgWireError::UserError(Box::new(ErrorInfo::new(
                        "FATAL".to_owned(),
                        "28P01".to_owned(),
                        format!("Invalid username `{}`", username).to_owned(),
                    ))));
                }
                let salt = vec![0, 0, 0, 0];
                let password = &FATHERDUCK_CONFIG.password;
        
                let hash_password =
                    hash_md5_password(username, password, salt.as_ref());
                Ok(Password::new(Some(salt), hash_password.as_bytes().to_vec()))
            },
            None => {
                return Err(PgWireError::UserNameRequired);
            },
        }
    }
}

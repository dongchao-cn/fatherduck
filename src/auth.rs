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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_password() {
        let username = "fatherduck";
        let password = "fatherduck";
        let salt = vec![0, 0, 0, 0];
        let hash_password = hash_md5_password(username, password, salt.as_ref());
        assert_eq!(hash_password, "md5dfee6c201d33684e31b4add68eaca57f");
    }

    #[tokio::test]
    async fn test_get_password() {
        let login_info = LoginInfo::new(Some("fatherduck"), None, "".to_owned());
        let result = FatherDuckAuthSource.get_password(&login_info).await;
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().password(), "md5dfee6c201d33684e31b4add68eaca57f".as_bytes());
    }

    #[tokio::test]
    async fn test_get_password_invalid_username() {
        let login_info = LoginInfo::new(Some("other_username"), None, "".to_owned());
        let result = FatherDuckAuthSource.get_password(&login_info).await;
        assert_eq!(result.is_err(), true);
    }
}
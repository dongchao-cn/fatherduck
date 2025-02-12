use async_trait::async_trait;
use pgwire::api::auth::{AuthSource, LoginInfo, Password};
use pgwire::error::PgWireResult;
use pgwire::api::auth::md5pass::hash_md5_password;

pub struct FatherDuckAuthSource;

#[async_trait]
impl AuthSource for FatherDuckAuthSource {
    async fn get_password(&self, login_info: &LoginInfo) -> PgWireResult<Password> {
        println!("login info: {:?}", login_info);

        let salt = vec![0, 0, 0, 0];
        let password = "pencil";

        let hash_password =
            hash_md5_password(login_info.user().as_ref().unwrap(), password, salt.as_ref());
        Ok(Password::new(Some(salt), hash_password.as_bytes().to_vec()))
    }
}
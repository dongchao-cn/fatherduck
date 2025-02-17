use config::Config;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref FATHERDUCK_CONFIG: FatherDuckConfig = {
        get_config()
    };
}

#[derive(serde::Deserialize, Debug)]
pub struct FatherDuckConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub path: String,
}
pub static MEMORY_PATH: &str = ":memory:";

pub fn get_config() -> FatherDuckConfig {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("fatherduck"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("FATHERDUCK"))
        .build()
        .unwrap();

    let config = settings
        .try_deserialize::<FatherDuckConfig>()
        .unwrap();

    println!("config: {:?}", config);

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_fatherdb_config() {
        assert_eq!(FATHERDUCK_CONFIG.host, "127.0.0.1");
        assert_eq!(FATHERDUCK_CONFIG.port, 5432);
        assert_eq!(FATHERDUCK_CONFIG.username, "fatherduck");
        assert_eq!(FATHERDUCK_CONFIG.password, "fatherduck");
    }
}

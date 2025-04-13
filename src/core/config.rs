use envy;
use once_cell::sync::Lazy;
use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub db_path: SecretBox<String>,
}

pub static CONFIG: Lazy<Configuration> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    envy::from_env::<Configuration>().expect("Unable to load env variables")
});

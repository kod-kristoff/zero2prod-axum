
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("sqlite://{}.sqlite", self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}

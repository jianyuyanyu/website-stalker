use config::ConfigError;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Site {
    pub url: Url,
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub from: String,
    pub user_agent: Option<String>,

    pub sites: Vec<Site>,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut settings = config::Config::default();
        settings
            // Add in `./website-stalker.toml`, `./website-stalker.yaml`, ...
            .merge(config::File::with_name("website-stalker").required(false))?
            // Add in settings from the environment (with a prefix of WEBSITE_STALKER)
            // Eg.. `WEBSITE_STALKER_DEBUG=1 network-stalker` would set the `debug` key
            .merge(config::Environment::with_prefix("WEBSITE_STALKER"))?;

        let settings: Self = settings.try_into()?;
        settings.validate()?;
        Ok(settings)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        let from = &self.from;
        if !from.contains('@') || !from.contains('.') {
            return Err(ConfigError::Message(format!(
                "from doesnt look like an email address: {}",
                from
            )));
        }

        Ok(())
    }
}

#[test]
fn can_parse_example_config() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("src/example-config.yaml"))
        .unwrap();

    let settings = settings.try_into::<Settings>().expect("failed to parse");
    println!("{:?}", settings);
}

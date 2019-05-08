use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Exchange {
    pub ticket_reader_data: String
}

#[derive(Debug, Deserialize)]
pub struct RabbitMQ {
    pub address: String,
    pub pre_fetch_items: u16
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub rabbit_mq: RabbitMQ,
    pub exchange: Exchange
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.try_into()
    }
}
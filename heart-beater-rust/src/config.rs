use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub http: Option<Vec<ConfigHttpPing>>,
    pub s3: Option<Vec<ConfigS3Ping>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigHttpPing {
    pub target_url: String,
    pub cron: String,
    pub heartbeat_url: String,
    pub status: Option<Vec<u16>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigS3Ping {
    pub region: String,
    pub bucket: String,
    pub prefix: String,
    pub cron: String,

    #[serde(with = "parse_duration")]
    pub grace: std::time::Duration,
    pub heartbeat_url: String,
    #[serde(with = "parse_min_size", default)]
    pub min_size: Option<u64>,
}

mod parse_duration {
    use serde::{de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<std::time::Duration, D::Error> {
        let duration: String = Deserialize::deserialize(deserializer)?;
        parse_duration::parse(&duration)
            .map_err(|e| Error::custom(format!("failed to parse duration {duration}. Erro:{e:?}")))
    }
}

mod parse_min_size {

    use serde::{de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        let bytes_maybe: Option<String> = Deserialize::deserialize(deserializer)?;
        match bytes_maybe {
            Some(bytes) => Ok(Some(parse_size::parse_size(&bytes).map_err(|e| {
                Error::custom(format!("failed to parse bytes {bytes}. Erro:{e:?}"))
            })?)),
            None => Ok(None),
        }
    }
}

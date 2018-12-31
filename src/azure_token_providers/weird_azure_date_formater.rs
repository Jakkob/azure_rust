use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};

// 2018-12-20 23:35:19.367401
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

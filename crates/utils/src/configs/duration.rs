use chrono::Duration;
use serde::{Deserialize, Deserializer};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}
impl Unit {
    pub fn get_primary_suffix(&self) -> &'static str {
        match self {
            Unit::Milliseconds => "ms",
            Unit::Seconds => "s",
            Unit::Minutes => "m",
            Unit::Hours => "h",
            Unit::Days => "d",
        }
    }
    pub fn get_seconds_suffix() -> char {
        's'
    }
    pub fn get_minutes_suffix() -> char {
        'm'
    }
    pub fn get_hours_suffix() -> char {
        'h'
    }
    pub fn get_days_suffix() -> char {
        'd'
    }

    pub fn get_ms_suffix() -> &'static str {
        "ms"
    }
}
/// A wrapper around `chrono::Duration` that allows for deserializing from a string
///
/// | Duration Type | Suffix | Example |
/// |---------------|--------|---------|
/// | milliseconds  | ms     | "100ms" |
/// | Seconds       | s      | '100s"  |
/// | Minutes       | m      | "100m"  |
/// | Hours         | h      | "100h"  |
/// | Days          | d      | "10d"   |
///
/// # Examples in TOML
/// ```toml
/// duration_in_milliseconds = "100ms"
/// duration_in_seconds = "100s"
/// duration_in_minutes = "100m"
/// duration_in_hours = "100h"
/// duration_in_days = "10d"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigDuration {
    pub duration: Duration,
    pub unit: Unit,
}
impl From<Duration> for ConfigDuration {
    fn from(duration: Duration) -> Self {
        // TODO: Make this the biggest unit possible
        Self {
            duration,
            unit: Unit::Milliseconds,
        }
    }
}
impl From<ConfigDuration> for Duration {
    fn from(duration: ConfigDuration) -> Self {
        duration.duration
    }
}

impl Deref for ConfigDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.duration
    }
}

impl DerefMut for ConfigDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.duration
    }
}

impl AsRef<Duration> for ConfigDuration {
    fn as_ref(&self) -> &Duration {
        &self.duration
    }
}
impl AsMut<Duration> for ConfigDuration {
    fn as_mut(&mut self) -> &mut Duration {
        &mut self.duration
    }
}

impl serde::Serialize for ConfigDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.unit {
            Unit::Milliseconds => format!(
                "{}{}",
                self.duration.num_milliseconds(),
                self.unit.get_primary_suffix()
            ),
            Unit::Seconds => format!(
                "{}{}",
                self.duration.num_seconds(),
                self.unit.get_primary_suffix()
            ),
            Unit::Minutes => format!(
                "{}{}",
                self.duration.num_minutes(),
                self.unit.get_primary_suffix()
            ),
            Unit::Hours => format!(
                "{}{}",
                self.duration.num_hours(),
                self.unit.get_primary_suffix()
            ),
            Unit::Days => format!(
                "{}{}",
                self.duration.num_days(),
                self.unit.get_primary_suffix()
            ),
        }
        .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ConfigDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if string.is_empty() {
            return Ok(Self {
                duration: Duration::zero(),
                unit: Unit::Milliseconds,
            });
        }
        let (unit, duration) = if string.ends_with(Unit::get_ms_suffix()) {
            let string = string.trim_end_matches(Unit::get_ms_suffix());
            let duration_as_int = string.parse().map_err(serde::de::Error::custom)?;
            (Unit::Milliseconds, Duration::milliseconds(duration_as_int))
        } else {
            let unit = match string.chars().last() {
                Some('s') => Unit::Seconds,
                Some('m') => Unit::Minutes,
                Some('h') => Unit::Hours,
                Some('d') => Unit::Days,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid duration unit: {}",
                        string
                    )))
                }
            };
            let duration_as_int = string.parse().map_err(serde::de::Error::custom)?;
            let duration = match unit {
                Unit::Seconds => Duration::seconds(duration_as_int),
                Unit::Minutes => Duration::minutes(duration_as_int),
                Unit::Hours => Duration::hours(duration_as_int),
                Unit::Days => Duration::days(duration_as_int),
                _ => {
                    unreachable!("Invalid duration unit: {}", string)
                }
            };
            (unit, duration)
        };

        Ok(Self { duration, unit })
    }
}

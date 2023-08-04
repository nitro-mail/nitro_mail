use std::fmt::{Display, Formatter};

use sea_orm::ConnectOptions;
use serde::{Deserialize, Serialize};

use helper_macros::const_and_default_function;
use utils::Config;

use crate::database_config::mysql::MysqlSettings;
use crate::database_config::postgres::PostgresSettings;
use crate::database_config::sqlite::SQLiteSettings;

const_and_default_function!(DEFAULT_MAX_SIZE: u32 = 10);
const_and_default_function!(DEFAULT_MIN_CONNECTIONS: u32 = 2);

///
/// # Example
/// ```toml
/// [pool]
/// max_size = 10
/// min_connections = 2
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PoolConfig {
    #[serde(default = "default_max_size")]
    pub max_size: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    // TODO add these
    //     pub connect_timeout: Duration,
    //     pub idle_timeout: Duration,
    //     pub acquire_timeout: Duration,
    //     pub max_lifetime: Duration,
}
impl Default for PoolConfig {
    fn default() -> Self {
        PoolConfig {
            max_size: default_max_size(),
            min_connections: default_min_connections(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub database: Database,
    pub pool: PoolConfig,
}

impl Into<ConnectOptions> for DatabaseConfig {
    fn into(self) -> ConnectOptions {
        let mut connect_options: ConnectOptions = match self.database {
            Database::Mysql(mysql) => mysql.to_string().into(),
            Database::Postgres(postgres) => postgres.to_string().into(),
            Database::SQLite(sql) => sql.to_string().into(),
        };

        connect_options
            .max_connections(self.pool.max_size)
            .min_connections(self.pool.min_connections);
        connect_options
    }
}
impl Config for DatabaseConfig {
    fn config_header() -> &'static str
    where
        Self: Sized,
    {
        // TODO put URL to documentation here
        r#""#
    }
    fn config_name() -> &'static str
    where
        Self: Sized,
    {
        "database.directory.toml"
    }
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            database: Database::default(),
            pool: PoolConfig::default(),
        }
    }
}
/// Database configuration
///
/// ## Display
/// On call to Display it will format it to a database url
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "settings")]
pub enum Database {
    Mysql(MysqlSettings),
    Postgres(PostgresSettings),
    SQLite(SQLiteSettings),
}
impl Default for Database {
    fn default() -> Self {
        Database::Postgres(PostgresSettings::default())
    }
}
impl Display for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Database::Mysql(mysql) => Display::fmt(mysql, f),
            Database::Postgres(postgres) => Display::fmt(postgres, f),
            Database::SQLite(sql) => Display::fmt(sql, f),
        }
    }
}

pub mod mysql {
    use std::fmt::{Display, Formatter};

    use serde::{Deserialize, Serialize};

    ///
    /// ## Display
    /// On call to Display it will format it to a database url
    /// ## Mysql Example
    /// ```toml
    /// [database]
    /// type = "mysql"
    /// [database.settings]
    /// username = "root"
    /// password = "password"
    /// host = "localhost::3306"
    /// database = "nitro_mail"
    /// ```
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct MysqlSettings {
        pub user: String,
        pub password: String,
        pub host: String,
        pub database: String,
    }
    impl Default for MysqlSettings {
        fn default() -> Self {
            MysqlSettings {
                user: "<MYSQL_USER>".to_string(),
                password: "<MYSQL_PASSWORD>".to_string(),
                host: "localhost:3306".to_string(),
                database: "nitro_mail".to_string(),
            }
        }
    }
    /// On call to Display it will format it to a database url
    impl Display for MysqlSettings {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "mysql://{}:{}@{}/{}",
                self.user, self.password, self.host, self.database
            )
        }
    }
}

pub mod postgres {
    use std::fmt::{Display, Formatter};

    use serde::{Deserialize, Serialize};

    /// ## Display
    /// On call to Display it will format it to a database url
    /// ## Postgres Example
    /// ```toml
    /// [database]
    /// type = "postgres"
    /// [database.settings]
    /// username = "postgres"
    /// password = "password"
    /// host = "localhost:5432"
    /// database = "nitro_mail"
    /// ```
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PostgresSettings {
        pub user: String,
        pub password: String,
        pub host: String,
        pub database: String,
    }
    impl Default for PostgresSettings {
        fn default() -> Self {
            PostgresSettings {
                user: "<POSTGRES_USER>".to_string(),
                password: "<POSTGRES_PASSWORD>".to_string(),
                host: "localhost:5432".to_string(),
                database: "nitro_mail".to_string(),
            }
        }
    }
    /// On call to Display it will format it to a database url
    impl Display for PostgresSettings {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "postgres://{}:{}@{}/{}",
                self.user, self.password, self.host, self.database
            )
        }
    }
}
pub mod sqlite {
    use std::fmt::{Display, Formatter};
    use std::path::PathBuf;

    use serde::{Deserialize, Serialize};

    /// ## SQLite File Example
    /// ```toml
    /// [database]
    /// type = "sqlite"
    /// [database.settings]
    /// type = "file"
    /// [database.settings.settings]
    /// path = "database.sqlite"
    /// ```
    /// ## SQLite Memory Example
    /// ```toml
    /// [database]
    /// type = "sqlite"
    /// [database.settings]
    /// type = "memory"
    /// ```
    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(tag = "type", content = "settings")]
    pub enum SQLiteSettings {
        Path(PathBuf),
        Memory,
    }
    impl Default for SQLiteSettings {
        fn default() -> Self {
            SQLiteSettings::Path(PathBuf::from("database.sqlite"))
        }
    }
    impl Display for SQLiteSettings {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                SQLiteSettings::Path(path) => write!(
                    f,
                    "sqlite://{}",
                    path.to_str().ok_or_else(|| std::fmt::Error::default())?
                ),
                SQLiteSettings::Memory => write!(f, "sqlite::memory:"),
            }
        }
    }
}

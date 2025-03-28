use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]  // <-- Ensures this struct is created even if missing
    pub config: GeneralConfig,
    
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            config: GeneralConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct GeneralConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_qfs_path")]
    pub qfs_path: String,
}

// Default values
fn default_host() -> String { "localhost".to_string() }
fn default_qfs_path() -> String { "/var/spool/quick_fox_status".to_string() }

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let default_paths = vec![
            "/etc/quick_fox_status/config".to_string(),
            format!("{}/.config/quick_fox_status/config", env::var("HOME").unwrap_or_default()),
        ];

        let config_paths: Vec<String> = env::var("FHS_TN_CONFIG_PATH")
            .map(|p| p.split(':').map(String::from).collect())  
            .unwrap_or(default_paths);

        // Start with default values
        let mut builder = Config::builder()
            .set_default("config.host", default_host())?
            .set_default("config.qfs_path", default_qfs_path())?;

        // Load config files
        for path in &config_paths {
            builder = builder.add_source(File::with_name(path).required(false));
        }

        // Add current directory .myapp.toml file
        builder = builder.add_source(File::with_name(".quick_fox_status").required(false));

        // Load environment variables with prefix "MYAPP_"
        builder = builder.add_source(Environment::with_prefix("QUICK_FOX_STATUS").separator("_"));

        let settings = builder.build()?.try_deserialize()?;

        Ok(settings)
    }
}


use config::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host_ip: String,
    pub host_port: i64,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {

  let env: String = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
  let env_config_file = format!("../config/config_{}.toml", env);

  let cfg = config::Config::builder()
    .add_source(File::with_name(&env_config_file).required(true))
    .build()?;

  Ok(AppConfig { 
    host_ip: cfg.get("host_ip")?,
    host_port: cfg.get_int("host_port")?,
  })
}
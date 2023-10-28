use serde::Deserialize;
use std::net::IpAddr;
use std::process;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        let cfg = match envy::from_env::<Config>() {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("failed to load env. {}",err);
                process::exit(1);
            }
        };
        cfg
    }

    pub fn parse_addr_and_port(&self) -> Result<(IpAddr, u16), std::net::AddrParseError> {
        let ip_addr = self.host.parse::<IpAddr>()?;
        let port = self.port;
        Ok((ip_addr, port))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    #[test]
    fn test_config_new() {
        let cfg = Config::new();
        assert_eq!(cfg.host, "127.0.0.1");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.database_url, "mongodb://root:password@localhost:27017");
    }

    #[test]
    fn test_config_parse_addr_and_port() {
        let cfg = Config::new();
         let(ip_addr, port) =  cfg.parse_addr_and_port().expect("failed to parse cfg.");
        assert_eq!(ip_addr.is_ipv4(), true);
        assert_eq!(ip_addr.to_string(), "127.0.0.1");
        assert_eq!(port, 8080);
    }
}

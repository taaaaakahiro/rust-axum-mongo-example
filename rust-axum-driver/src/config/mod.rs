use serde::Deserialize;

use std::process;

#[derive(Deserialize, Debug)]

pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        let config = match envy::from_env::<Config>() {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        };
        config
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_load_config() {
    //     let cfg: Config = load_confg();
    //     assert_eq!(cfg.port, 8080);
    // }
}

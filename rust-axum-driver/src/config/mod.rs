use serde::{Deserialize};

use std::{
    process
};

#[derive(Deserialize, Debug)]
pub struct Config {
   pub port: u16,
}

pub fn load_confg()->Config{
    let config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    config
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, load_confg};

    #[test]
    fn test_load_config() {
        let cfg: Config = load_confg();
        assert_eq!(cfg.port, 8080);
    }
}




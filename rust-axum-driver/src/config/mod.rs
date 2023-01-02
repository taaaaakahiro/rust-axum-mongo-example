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
    #[test]
    fn load_config() {
        assert_eq!(2 + 2, 4);
    }
}




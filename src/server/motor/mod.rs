use std::env;
use std::fs;
use std::io;

mod motor {

    pub struct brushless {
        active: bool,
        name: &str,
        port: u64,
    }

    pub struct stepper {
        active: bool,
        name: &str,
        port: u64,
    }

    pub fn updateMotorIndex() {

        let contents = fs::read_to_string("src/server/motor/config.toml")
            .expect("Something went wrong reading the file");

    }

}


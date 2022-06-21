use std::env;
use std::fs;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use telnet::{Telnet, Event};

const VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {

    println!("rustyluna ttyl client v{}", VERSION);
    let contents = fs::read_to_string("src/client/art.txt")
        .expect("Something went wrong reading the file");
    println!("\n{}", contents);

    let mut address = String::new();

    loop {
        match ask_to_connect(&mut address) {
            None => {
                println!("{}", "Word. No extra available client modules in this junt. Closing.".to_string());
                break;
            },
            Some(x) => {
                println!("{}", format!("{}{}", "Connecting to: ".to_string(), x));
                
                let server = address.parse::<IpAddr>().unwrap();

                let mut connection = Telnet::connect((server, 23), 256)
                .expect("Couldn't connect to the server...");
            },
        }
    }

//connect
//send stick codes
//ask for commands
//send command codes
}

fn ask_to_connect(addy: &mut String) -> Option<&String>{

    println!("Connect to server? (Y/n)");
    let mut conf = String::new();
    io::stdin().read_line(&mut conf).ok()?;
 
    if conf.trim() == "n" {
        return None;
    } else {
        println!("addr :");
        io::stdin().read_line(addy).ok()?;
        return Some(addy);    
    }
}





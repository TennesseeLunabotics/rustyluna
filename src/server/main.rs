#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate tokio_core;

use tarpc::sync::{client, server};
use tarpc::sync::client::ClientExt;
use tarpc::util::Never;
use tokio_core::reactor;
use std::sync::mpsc;
use std::thread;

mod motor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

service!{
    rpc info() -> &str;
}

#[derive(Clone)]
struct headlessServer;

impl SyncService for headlessServer {
    fn info(&self) -> Result<&str, Never> {
        Ok(format!("rustyluna headless server v{}", VERSION))
    }
}


fn main() {

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut handle = headlessServer.listen("localhost:10000",
            server::Options::default()).unwrap();
        tx.send(handle.addr()).unwrap();
        handle.run();
    });

    let addr = rx.recv().unwrap();
    let client = SyncClient::connect(addr, client::Options::default()).unwrap();
    println!("{}", client.info()); //testing tarpc
    
    //listen for connections
    //accept a connection
    //refuse others until disconnect

    //loop listen for remote commands and serial data
    //loop interpret serial data
    //loop send serial data

    let ports = serialport::available_ports().expect("No ports founds.");
    for p in ports {
        println!("{}", p.port_name)
    }

    

}

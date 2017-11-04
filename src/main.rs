extern crate ansi_term;
extern crate clap;

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::{Read, Write};
use ansi_term::Colour::Red;
use clap::{App, Arg};

fn main() {
    let matches = App::new("tcping")
        .version("0.1.0")
        .author("Yilin Chen <sticnarf@gmail.com>")
        .about("Ping over a TCP connection")
        .arg(
            Arg::with_name("host")
                .value_name("host")
                .required(true)
                .hidden(true),
        )
        .arg(
            Arg::with_name("port")
                .value_name("port")
                .required(true)
                .hidden(true)
                .validator(|port| {
                    port.parse::<u16>()
                        .map_err(|_| "Invalid port".to_owned())
                        .map(|_| ())
                }),
        )
        .arg(
            Arg::with_name("timeout")
                .value_name("timeout")
                .short("w")
                .takes_value(true)
                .default_value("1000")
                .help("Time to wait for a response, in milliseconds"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();
    let addr = match format!("{}:{}", host, port).to_socket_addrs() {
        Ok(mut addrs) => addrs.filter(|addr| addr.is_ipv4()).next().unwrap(),
        Err(_) => {
            println!("{} Unknown host", Red.paint("error:"));
            return;
        }
    };
    println!("{}", addr);
    let timeout = match matches
        .value_of("timeout")
        .unwrap()
        .parse()
        .map(|t| Duration::from_millis(t))
    {
        Ok(timeout) => timeout,
        Err(_) => {
            println!("{} Invalid timeout value", Red.paint("error:"));
            return;
        }
    };

    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(c) => if c.peer_addr().is_ok() {
            println!("{} port {} open", host, port)
        } else {
            println!("{} port {} closed", host, port)
        },
        Err(e) => println!("{} port {} {}", host, port, e),
    }
}

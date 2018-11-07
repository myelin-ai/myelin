#![feature(const_ip)]

use clap::{App, Arg};
use myelin_visualization_server::start_server;
use std::net::{IpAddr, Ipv6Addr};

struct Arguments {
    host: IpAddr,
    port: u16,
}

fn parse_arguments() -> Arguments {
    let matches = App::new("Visualization Server")
        .about("Runs a websocket server which provides the in-browser visualization with data")
        .arg(
            Arg::with_name("port")
                .short("P")
                .long("port")
                .value_name("PORT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("address")
                .short("H")
                .long("host")
                .value_name("HOST")
                .takes_value(true),
        )
        .get_matches();

    const DEFAULT_PORT: u16 = 6956;
    let port = matches
        .value_of("port")
        .map(|port| port.parse().expect("port must be a valid port number"))
        .unwrap_or(DEFAULT_PORT);

    const DEFAULT_HOST: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let host = matches
        .value_of("host")
        .map(|host| {
            host.parse()
                .expect("host must be a valid ipv4 or ipv6 address")
        })
        .unwrap_or(DEFAULT_HOST);

    Arguments { host, port }
}

fn main() {
    let arguments = parse_arguments();

    simple_logger::init().unwrap();

    start_server((arguments.host, arguments.port));
}

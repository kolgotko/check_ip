extern crate ipnetwork;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_json;

use clap::App;

use std::net::*;

use std::process::*;

use ipnetwork::*;

fn ping4(ip: Ipv4Addr) -> ExitStatus {

    let mut child = Command::new("ping")
        .arg("-c 1")
        .arg("-i 0")
        .arg("-t 1")
        .arg("-o")
        .arg(format!("{}", ip))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("error");

    let ecode = child.wait()
        .expect("error");

    ecode

}

fn ping6(ip: Ipv6Addr) -> ExitStatus {

    let mut child = Command::new("ping6")
        .arg("-c 1")
        .arg("-i 0")
        .arg("-X 1")
        .arg("-o")
        .arg(format!("{}", ip))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("error");

    let ecode = child.wait()
        .expect("error");

    ecode

}

fn get_free4(ip: &str) -> Option<Ipv4Addr> {

    let network: Ipv4Network = ip.parse().unwrap();
    let network_addr = network.network();
    let broadcast = network.broadcast();

    let mut result = None;

    for addr in network.iter() {

        if addr == broadcast || addr == network_addr {
            continue; 
        }

        let ecode = ping4(addr);

        match ecode.code().unwrap() {

            0 => continue,
            2 => {
                result = Some(addr);
                break;
            },
            _ => break

        };

    }

    result

}

fn get_free6(ip: &str) -> Option<Ipv6Addr> {

    let network: Ipv6Network = ip.parse().unwrap();
    let network_addr = network.network();
    let mut result = None;

    for addr in network.iter() {

        if addr == network_addr { continue; }

        let ecode = ping6(addr);

        match ecode.code().unwrap() {

            0 => continue,
            2 => {
                result = Some(addr);
                break;
            },
            _ => break

        };

    }

    result

}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let v4 = matches.value_of("ipv4");
    let v6 = matches.value_of("ipv6");

    let free6 = match v6 {
        Some(ip) => get_free6(ip),
        None => None
    };

    let free4 = match v4 {
        Some(ip) => get_free4(ip),
        None => None
    };

    match matches.occurrences_of("json") {
        1 => {
            let j = json!({
                "free4": v4.unwrap_or(""),
                "free6": v6.unwrap_or(""),
            });

            println!("{}", j);
        },
        _ => ()
    };

}

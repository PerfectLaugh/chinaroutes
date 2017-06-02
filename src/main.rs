extern crate clap;
extern crate libc;

use clap::{Arg, App};

use std::str::FromStr;
use std::net::Ipv4Addr;

use libc::{int32_t, uint8_t, uint32_t, c_void};

mod parse;

type Ipv4 = [uint8_t; 4];

#[repr(C)]
struct route_entry {
    dst: Ipv4,
    mask: uint32_t,
}

#[link(name="routes", kind="static")]
extern {
    fn netlink_route_connect(skptr: *mut *const c_void) -> int32_t;
    fn netlink_route_disconnect(sk: *const c_void);
    fn netlink_route_add(sk: *const c_void, table: uint32_t, via: *const Ipv4, rtentry: *const route_entry) -> int32_t;
    fn netlink_route_del(sk: *const c_void, table: uint32_t, via: *const Ipv4, rtentry: *const route_entry) -> int32_t;
}

fn main() {
    let matches = App::new("chinaroutes")
                    .version("0.1.1")
                    .author("PerfectLaugh")
                    .about("Add china routes for personal routing")
                    .arg(Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .help("Input/Output file")
                        .value_name("FILE")
                        .required(true))
                    .arg(Arg::with_name("table")
                        .short("t")
                        .long("table")
                        .help("Routing table")
                        .value_name("TABLE"))
                    .arg(Arg::with_name("target")
                        .long("target")
                        .help("Target router")
                        .value_name("ROUTER")
                        .required(true))
                    .arg(Arg::with_name("delete")
                        .long("delete")
                        .help("Delete the entries (Default to add)"))
                    .arg(Arg::with_name("country")
                        .long("country")
                        .short("c")
                        .help("Target country (case-sensitive)")
                        .value_name("COUNTRY")
                        .required(true))
                    .get_matches();

    let file = matches.value_of("file").unwrap();
    let target = matches.value_of("target").unwrap();
    let target_country = matches.value_of("country").unwrap();
    let targetip = match Ipv4Addr::from_str(target) {
        Ok(ip) => ip,
        Err(err) => {
            println!("Target IP format error: {}", err);
            std::process::exit(1);
        },
    }.octets();
    let table = matches.value_of("table").unwrap_or("254");
    let tableid: u32 = match table.to_string().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Table id error");
            std::process::exit(1);
        },
    };
    let addflag = !matches.is_present("delete");
    
    let ips = parse::parse_ip(file, target_country);

    let mut sk: *const c_void = std::ptr::null();
    let ret;
    unsafe {
        ret = netlink_route_connect(&mut sk as *mut *const c_void);
    }
    if ret != 0 {
        println!("Cannot connect netlink: {}", ret);
        std::process::exit(2);
    }
    for ip in ips {
        let rtentry = route_entry {
            dst: ip.0,
            mask: ip.1,
        };
        if addflag {
            unsafe {
                netlink_route_add(sk, tableid, &targetip, &rtentry);
            }
        } else {
            unsafe {
                netlink_route_del(sk, tableid, &targetip, &rtentry);
            }
        }
    }
    unsafe {
        netlink_route_disconnect(sk);
    }
}

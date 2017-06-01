extern crate regex;

use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use self::regex::Regex;

use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn parse_ip(file: &str, target_country: &str) -> Vec<([u8; 4], u32)> {
    let path = Path::new(file);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", display, why);
            exit(2);
        },
        Ok(file) => file,
    };

    let mut bufreader = BufReader::new(file);
    let mut s = String::new();
    match bufreader.read_to_string(&mut s) {
        Err(why) => {
            println!("Couldn't read {}: {}", display, why);
            exit(2);
        },
        Ok(_) => (),
    }

    let re = Regex::new(r"apnic\|(\w+)\|ipv4\|[0-9\.]+\|[0-9]+\|[0-9]+\|a.*").unwrap();

    let mut results: Vec<([u8; 4], u32)> = vec![];
    for mat in re.find_iter(&s) {
        let caps = re.captures(mat.as_str()).unwrap();
        let country = &caps[1];

        if country == target_country {
            let unit_items: Vec<&str> = mat.as_str().split('|').collect();
            let ip_str = unit_items[3];
            let ip = Ipv4Addr::from_str(ip_str).unwrap();
            let num_ip: u32 = unit_items[4].to_string().parse().unwrap();

            let imask = 32 - ((num_ip as f64).log(2.0) as u32);

            results.push((ip.octets(), imask));
        }
    }

    println!("{} found", results.len());

    results
}